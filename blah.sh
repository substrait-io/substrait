#!/bin/bash

# Start Trino in background
docker run --rm -d --name trino-test trinodb/trino:latest > /dev/null 2>&1
sleep 20

run_duckdb() {
    docker run --rm duckdb/duckdb:latest duckdb -noheader -list -c "$1" 2>/dev/null
}

run_spark() {
    result=$(docker run --rm apache/spark:latest /opt/spark/bin/spark-sql -e "$1" 2>&1 | grep -v "WARN\|INFO\|Time taken\|Spark Web\|Spark master\|ObjectStore" | grep -v "^$" | tail -1)
    if [ -z "$result" ]; then echo "NULL"; else echo "$result"; fi
}

run_trino() {
    result=$(docker exec trino-test trino --execute "$1" 2>&1 | grep -v "WARNING\|jline" | tr -d '"')
    if [ -z "$result" ]; then echo "NULL"; else echo "$result"; fi
}

echo "=== CARDINALITY ==="
echo "cardinality([1, NULL, 3]): DuckDB=$(run_duckdb 'SELECT len([1, NULL, 3])'), Spark=$(run_spark 'SELECT cardinality(array(1, NULL, 3))')"
echo "cardinality(NULL): DuckDB=$(run_duckdb 'SELECT len(NULL::INT[])'), Spark=$(run_spark 'SELECT cardinality(NULL)')"

echo ""
echo "=== SORT ==="
echo "sort([3, NULL, 1]): DuckDB=$(run_duckdb 'SELECT list_sort([3, NULL, 1])'), Spark=$(run_spark 'SELECT array_sort(array(3, NULL, 1))')"

echo ""
echo "=== ANY_MATCH ==="
echo "any_match([1,2,3], x->x>2): Spark=$(run_spark 'SELECT exists(array(1, 2, 3), x -> x > 2)'), Trino=$(run_trino 'SELECT any_match(ARRAY[1, 2, 3], x -> x > 2)')"
echo "any_match([1,NULL,2], x->x>5): Spark=$(run_spark 'SELECT exists(array(1, NULL, 2), x -> x > 5)'), Trino=$(run_trino 'SELECT any_match(ARRAY[1, NULL, 2], x -> x > 5)')"

echo ""
echo "=== ALL_MATCH ==="
echo "all_match([1,2,3], x->x>0): Spark=$(run_spark 'SELECT forall(array(1, 2, 3), x -> x > 0)'), Trino=$(run_trino 'SELECT all_match(ARRAY[1, 2, 3], x -> x > 0)')"
echo "all_match([1,NULL,2], x->x>0): Spark=$(run_spark 'SELECT forall(array(1, NULL, 2), x -> x > 0)'), Trino=$(run_trino 'SELECT all_match(ARRAY[1, NULL, 2], x -> x > 0)')"

docker stop trino-test > /dev/null 2>&1
