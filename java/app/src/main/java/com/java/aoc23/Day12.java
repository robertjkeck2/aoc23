/*
 * Advent of Code 2023 Day 12
 */
package com.java.aoc23;

public class Day12 implements Day {
    public void part1(String input) {
        SpringReport springReport = new SpringReport(input);
        int sum = springReport.getNumPermutations();
        System.out.println("Part 1: " + sum);
    }

    public void part2(String input) {
        System.out.println("Part 2: " + "TODO");
    }
}

final class SpringReport {
    private SpringRow[] springRows;

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

    public int getNumPermutations() {
        int sum = 0;
        for (SpringRow springRow : springRows) {
            sum += springRow.getRowPermutations(springRow.getRow(), springRow.getRowGroups());
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

    public String getRow() {
        return row.split(" ")[0];
    }

    public int getRowPermutations(String rowString, int[] rowGroups) {
        if (rowString.isBlank()) {
            if (rowGroups.length == 0) {
                return 1;
            } else {
                return 0;
            }
        }

        int permutations = 0;
        if (rowString.startsWith(".")) {
            permutations += getRowPermutations(rowString.substring(1), rowGroups);
        } else if (rowString.startsWith("?")) {
            permutations += getRowPermutations("." + rowString.substring(1), rowGroups)
                    + getRowPermutations("#" + rowString.substring(1), rowGroups);
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
                        permutations = getRowPermutations(rowString.substring(damaged + 1), newGroups);
                    } else if (rowString.charAt(damaged) == '?') {
                        permutations = getRowPermutations("." + rowString.substring(damaged + 1), newGroups);
                    }
                }
            }
        }

        return permutations;
    }
}