#!/bin/bash
# Script to show PR open and merge rates for a GitHub repository

set -e

# Configuration
DAYS=${1:-90}  # Default to last 90 days
REPO=${2:-""}  # Use current repo if not specified

echo "Fetching PR data for the last $DAYS days..."
echo

# Calculate date range (GNU date syntax)
START_DATE=$(date -d "-$DAYS days" +%Y-%m-%d)
END_DATE=$(date +%Y-%m-%d)

# Fetch opened PRs
echo "Fetching opened PRs..."
OPENED_PRS=$(gh pr list --state all --search "created:>=$START_DATE" --json number,createdAt,mergedAt,state --limit 1000 $REPO)

# Fetch merged PRs (subset of opened or older PRs merged in this period)
echo "Fetching merged PRs..."
MERGED_PRS=$(gh pr list --state merged --search "merged:>=$START_DATE" --json number,createdAt,mergedAt --limit 1000 $REPO)

# Count totals
TOTAL_OPENED=$(echo "$OPENED_PRS" | jq 'length')
TOTAL_MERGED=$(echo "$MERGED_PRS" | jq 'length')

# Calculate rates
WEEKS=$(echo "scale=2; $DAYS / 7" | bc)
OPENED_PER_WEEK=$(echo "scale=2; $TOTAL_OPENED / $WEEKS" | bc)
MERGED_PER_WEEK=$(echo "scale=2; $TOTAL_MERGED / $WEEKS" | bc)

# Calculate average time to merge (for PRs merged in this period)
AVG_MERGE_TIME=$(echo "$MERGED_PRS" | jq -r '
  [.[] | select(.mergedAt != null and .createdAt != null) |
    (((.mergedAt | fromdateiso8601) - (.createdAt | fromdateiso8601)) / 86400)
  ] | if length > 0 then (add / length) else 0 end
')

echo
echo "========================================"
echo "PR Statistics ($START_DATE to $END_DATE)"
echo "========================================"
echo
printf "%-25s %s\n" "Period:" "$DAYS days ($WEEKS weeks)"
printf "%-25s %s\n" "PRs Opened:" "$TOTAL_OPENED"
printf "%-25s %s\n" "PRs Merged:" "$TOTAL_MERGED"
echo
echo "--- Rates ---"
printf "%-25s %.2f PRs/week\n" "Open Rate:" "$OPENED_PER_WEEK"
printf "%-25s %.2f PRs/week\n" "Merge Rate:" "$MERGED_PER_WEEK"
printf "%-25s %.1f days\n" "Avg Time to Merge:" "$AVG_MERGE_TIME"
echo

# Weekly breakdown
echo "--- Weekly Breakdown ---"
echo
printf "%-12s %10s %10s\n" "Week" "Opened" "Merged"
printf "%-12s %10s %10s\n" "----" "------" "------"

for ((i=$DAYS; i>0; i-=7)); do
    WEEK_START=$(date -d "-$i days" +%Y-%m-%d)
    WEEK_END=$(date -d "-$((i-7 > 0 ? i-7 : 0)) days" +%Y-%m-%d)
    
    WEEK_OPENED=$(echo "$OPENED_PRS" | jq --arg start "$WEEK_START" --arg end "$WEEK_END" '
      [.[] | select(.createdAt >= $start and .createdAt < $end)] | length
    ')
    
    WEEK_MERGED=$(echo "$MERGED_PRS" | jq --arg start "$WEEK_START" --arg end "$WEEK_END" '
      [.[] | select(.mergedAt >= $start and .mergedAt < $end)] | length
    ')
    
    printf "%-12s %10s %10s\n" "$WEEK_START" "$WEEK_OPENED" "$WEEK_MERGED"
done

echo
echo "Done!"
