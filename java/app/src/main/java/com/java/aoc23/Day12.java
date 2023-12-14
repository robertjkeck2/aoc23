/*
 * Advent of Code 2023 Day 12
 */
package com.java.aoc23;

import java.util.HashMap;

public class Day12 implements Day {
    public void part1(String input) {
        HashMap<RowPermutation, Long> memo = new HashMap<>();
        SpringReport springReport = new SpringReport(input, memo);
        long sum = springReport.getNumPermutations(false);
        System.out.println("Part 1: " + sum);
    }

    public void part2(String input) {
        HashMap<RowPermutation, Long> memo = new HashMap<>();
        SpringReport springReport = new SpringReport(input, memo);
        long sum = springReport.getNumPermutations(true);
        System.out.println("Part 2: " + sum);
    }
}

final record RowPermutation(String row, int[] rowGroups) {
    @Override
    public int hashCode() {
        return row.hashCode() + rowGroups.hashCode();
    }

    @Override
    public boolean equals(Object obj) {
        if (obj instanceof RowPermutation) {
            RowPermutation other = (RowPermutation) obj;
            return row.equals(other.row) && rowGroups.equals(other.rowGroups);
        }
        return false;
    }
}

final class SpringReport {
    private SpringRow[] springRows;
    private HashMap<RowPermutation, Long> memo;

    public SpringReport(String input, HashMap<RowPermutation, Long> memo) {
        String[] rows = input.split("\n");
        this.memo = memo;
        springRows = new SpringRow[rows.length];
        for (int i = 0; i < rows.length; i++) {
            springRows[i] = new SpringRow(rows[i]);
        }
    }

    public SpringRow[] getSpringRows() {
        return springRows;
    }

    public long getNumPermutations(boolean expanded) {
        long sum = 0;
        for (SpringRow springRow : springRows) {
            sum += springRow.getRowPermutations(expanded ? springRow.getExpandedRow() : springRow.getRow(),
                    expanded ? springRow.getExpandedRowGroups() : springRow.getRowGroups(), this.memo);
        }
        return sum;

    }
}

final class SpringRow {
    private String row;

    public SpringRow(String row) {
        this.row = row;
    }

    public int[] getRowGroups() {
        String[] rowGroups = row.split(" ")[1].split(",");
        int[] rowGroupsInt = new int[rowGroups.length];
        for (int i = 0; i < rowGroups.length; i++) {
            rowGroupsInt[i] = Integer.parseInt(rowGroups[i]);
        }
        return rowGroupsInt;
    }

    public int[] getExpandedRowGroups() {
        int[] rowGroups = getRowGroups();
        int[] expandedRowGroups = new int[rowGroups.length * 5];
        for (int i = 0; i < expandedRowGroups.length; i++) {
            expandedRowGroups[i] = rowGroups[i % rowGroups.length];
        }
        return expandedRowGroups;
    }

    public String getRow() {
        return row.split(" ")[0];
    }

    public String getExpandedRow() {
        String expandedRow = "";
        String rowSplit = row.split(" ")[0];
        for (int i = 0; i < 5; i++) {
            expandedRow += rowSplit;
            expandedRow += i == 4 ? "" : "?";
        }
        return expandedRow;
    }

    public int getRowPermutations(String rowString, int[] rowGroups, HashMap<RowPermutation, Long> memo) {
        RowPermutation rowPermutation = new RowPermutation(rowString, rowGroups);

        if (memo.containsKey(rowPermutation)) {
            return memo.get(rowPermutation).intValue();
        }

        if (rowString.isBlank()) {
            if (rowGroups.length == 0) {
                return 1;
            } else {
                return 0;
            }
        }

        int permutations = 0;
        if (rowString.startsWith(".")) {
            permutations += getRowPermutations(rowString.substring(1), rowGroups, memo);
        } else if (rowString.startsWith("?")) {
            permutations += getRowPermutations("." + rowString.substring(1), rowGroups, memo)
                    + getRowPermutations("#" + rowString.substring(1), rowGroups, memo);
        } else {
            if (rowGroups.length > 0) {
                int damaged = rowGroups[0];
                if (damaged <= rowString.length()
                        && rowString.chars().limit(damaged).allMatch(c -> c == '#' || c == '?')) {
                    int[] newGroups = new int[rowGroups.length - 1];
                    for (int i = 0; i < newGroups.length; i++) {
                        newGroups[i] = rowGroups[i + 1];
                    }
                    if (damaged == rowString.length()) {
                        permutations = newGroups.length == 0 ? 1 : 0;
                    } else if (rowString.charAt(damaged) == '.') {
                        permutations = getRowPermutations(rowString.substring(damaged + 1), newGroups, memo);
                    } else if (rowString.charAt(damaged) == '?') {
                        permutations = getRowPermutations("." + rowString.substring(damaged + 1), newGroups, memo);
                    }
                }
            }
        }

        return permutations;
    }
}