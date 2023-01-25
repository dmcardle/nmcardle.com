#!/usr/bin/env python
#
import csv

if __name__ == '__main__':
    with open("gp-search-20230122-092746.csv") as f:
        # Skip the metadata line with the search URL.
        f.readline()

        # Skip the column name row.
        f.readline()

        reader = csv.reader(f)
        for row in reader:
            row = [s.strip() for s in row]
            id, title, assignee, inventor, prio_date, filing_date, pub_date, grant_date, result_link, fig_link = row

            print(f"* {inventor} ({grant_date}). *{title}* ({id}). [{result_link}]({result_link})")
