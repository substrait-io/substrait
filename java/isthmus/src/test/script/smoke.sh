#!/bin/bash
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "${parent_path}"
CMD=../../../build/graal/isthmus
LINEITEM="CREATE TABLE LINEITEM (L_ORDERKEY BIGINT NOT NULL, L_PARTKEY BIGINT NOT NULL, L_SUPPKEY BIGINT NOT NULL, L_LINENUMBER INTEGER, L_QUANTITY DECIMAL, L_EXTENDEDPRICE DECIMAL, L_DISCOUNT DECIMAL, L_TAX DECIMAL, L_RETURNFLAG CHAR(1), L_LINESTATUS CHAR(1), L_SHIPDATE DATE, L_COMMITDATE DATE, L_RECEIPTDATE DATE, L_SHIPINSTRUCT CHAR(25), L_SHIPMODE CHAR(10), L_COMMENT VARCHAR(44))"
echo $LINEITEM
#set -x

# Simple
$CMD 'select * from lineitem' --create "${LINEITEM}"

# With condition
$CMD 'select * from lineitem where l_orderkey > 10' --create "${LINEITEM}"

# Aggregate
$CMD 'select l_orderkey, count(l_partkey) from lineitem group by l_orderkey' --create "${LINEITEM}"
