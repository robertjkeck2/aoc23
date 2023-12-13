/*
 * Advent of Code 2023 Day 12
 */
package com.java.aoc23;

import java.util.HashMap;

import com.java.aoc23.SpringRow.RowPermutation;

public class Day12 implements Day {
    public void part1(String input) {
        SpringReport springReport = new SpringReport(input);
        long sum = springReport.getNumPermutations(false);
        System.out.println("Part 1: " + sum);
    }

    public void part2(String input) {
        SpringReport springReport = new SpringReport(input);
        springReport.getNumPermutations(false);
        long sum = springReport.getNumPermutations(true);
        System.out.println("Part 2: " + sum);
    }
}

final class SpringReport {
    private SpringRow[] springRows;
    private static final HashMap<RowPermutation, Integer> memoMap = new HashMap<>();

    public SpringReport(String input) {
        String[] rows = input.split("\n");
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
                    expanded ? springRow.getExpandedRowGroups() : springRow.getRowGroups(),
                    expanded ? new HashMap<RowPermutation, Integer>() : memoMap);
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

    public record RowPermutation(String row, int[] rowGroups) {
    }

    public int getRowPermutations(String rowString, int[] rowGroups, HashMap<RowPermutation, Integer> memoMap) {
        RowPermutation rowPermutation = new RowPermutation(rowString, rowGroups);
        if (memoMap.containsKey(rowPermutation)) {
            return memoMap.get(rowPermutation);
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
            permutations += getRowPermutations(rowString.substring(1), rowGroups, memoMap);
        } else if (rowString.startsWith("?")) {
            permutations += getRowPermutations("." + rowString.substring(1), rowGroups, memoMap)
                    + getRowPermutations("#" + rowString.substring(1), rowGroups, memoMap);
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
                        permutations = getRowPermutations(rowString.substring(damaged + 1), newGroups, memoMap);
                    } else if (rowString.charAt(damaged) == '?') {
                        permutations = getRowPermutations("." + rowString.substring(damaged + 1), newGroups, memoMap);
                    }
                }
            }
        }

        memoMap.put(rowPermutation, permutations);
        return permutations;
    }
}