/*
 * Advent of Code 2023 Day 14
 */
package com.java.aoc23;

import java.util.ArrayList;

public class Day14 implements Day {
    public void part1(String input) {
        Dish dish = new Dish(input);
        int weights = dish.getWeights(false);
        System.out.println("Part 1: " + weights);
    }

    public void part2(String input) {
        Dish dish = new Dish(input);
        int weights = dish.getWeights(true);
        System.out.println("Part 2: " + weights);
    }
}

final class Dish {
    private DishColumn[] rows;
    private DishColumn[] columns;

    public Dish(String input) {
        String[] inputRows = input.split("\n");
        rows = new DishColumn[inputRows.length];
        columns = new DishColumn[inputRows[0].length()];

        for (int i = 0; i < inputRows[0].length(); i++) {
            DishColumn row = new DishColumn(new ArrayList<String>());
            DishColumn column = new DishColumn(new ArrayList<String>());
            for (int j = 0; j < inputRows.length; j++) {
                row.addRock(inputRows[i].split("")[j]);
                column.addRock(inputRows[j].split("")[i]);
            }
            rows[i] = row;
            columns[i] = column;
        }
    }

    public void updateRows(DishColumn[] newColumns) {
        for (int i = 0; i < newColumns.length; i++) {
            for (int j = 0; j < newColumns[i].rocks.size(); j++) {
                rows[i].rocks.set(j, newColumns[i].rocks.get(j));
            }
        }
    }

    public void updateColumns(DishColumn[] newRows) {
        for (int i = 0; i < newRows.length; i++) {
            for (int j = 0; j < newRows[i].rocks.size(); j++) {
                columns[i].rocks.set(j, newRows[i].rocks.get(j));
            }
        }
    }

    public void shiftVertical(boolean up) {
        if (up) {
            for (int i = 0; i < columns.length; i++) {
                columns[i].sort();
                columns[i] = new DishColumn(columns[i].sortedRocks);
            }
        } else {
            for (int i = 0; i < columns.length; i++) {
                columns[i].reverse();
                columns[i].sort();
                columns[i] = new DishColumn(columns[i].sortedRocks);
            }
        }
        updateRows(columns);
        updateColumns(rows);
    }

    public void shiftHorizontal(boolean left) {
        if (left) {
            for (int i = 0; i < rows.length; i++) {
                rows[i].sort();
                rows[i] = new DishColumn(rows[i].sortedRocks);
            }
        } else {
            for (int i = 0; i < rows.length; i++) {
                rows[i].reverse();
                rows[i].sort();
                rows[i] = new DishColumn(rows[i].sortedRocks);
            }
        }
        updateColumns(rows);
        updateRows(columns);
    }

    public void runCycle(int numCycles) {
        for (int i = 0; i < numCycles; i++) {
            shiftVertical(true);
            shiftHorizontal(true);
            shiftVertical(false);
            shiftHorizontal(false);
        }
    }

    public int getWeights(boolean runCycle) {
        int weights = 0;
        if (runCycle) {
            runCycle(1000000000);
        } else {
            for (int i = 0; i < columns.length; i++) {
                columns[i].sort();
            }
        }
        for (int i = 0; i < columns.length; i++) {
            weights += columns[i].calculateScore(runCycle);
        }
        return weights;
    }
}

final class DishColumn {
    public ArrayList<String> rocks;
    public ArrayList<String> sortedRocks;

    public DishColumn(ArrayList<String> rocks) {
        this.rocks = rocks;
        this.sortedRocks = new ArrayList<String>();
    }

    public void addRock(String rock) {
        rocks.add(rock);
    }

    public void reverse() {
        ArrayList<String> newRocks = new ArrayList<String>();
        for (int i = rocks.size() - 1; i >= 0; i--) {
            newRocks.add(rocks.get(i));
        }
        rocks = newRocks;
    }

    public void sort() {
        if (rocks.size() == 0) {
            rocks = sortedRocks;
            return;
        }
        String firstRock = rocks.get(0);
        if (firstRock.equals("O") || firstRock.equals("#")) {
            sortedRocks.add(firstRock);
            rocks.remove(0);
            sort();
        } else {
            int stopNum = 0;
            boolean isO = false;
            for (int i = 0; i < rocks.size() - 1; i++) {
                if (rocks.get(i + 1).equals("#")) {
                    stopNum = i + 1;
                    break;
                } else if (rocks.get(i + 1).equals("O")) {
                    isO = true;
                    sortedRocks.add(rocks.get(i + 1));
                    rocks.set(i + 1, ".");
                    rocks.remove(0);
                    break;
                }
            }
            if (stopNum > 0 || isO) {
                for (int i = 0; i < stopNum; i++) {
                    sortedRocks.add(".");
                    rocks.remove(0);
                }
                sort();
            } else {
                sortedRocks.add(".");
                rocks.remove(0);
                sort();
            }
        }
    }

    public int calculateScore(boolean runCycle) {
        int score = 0;
        ArrayList<String> rocksToCalculate = runCycle ? rocks : sortedRocks;
        for (int i = 0; i < rocksToCalculate.size(); i++) {
            if (rocksToCalculate.get(i).equals("O")) {
                score += rocksToCalculate.size() - i;
            }
        }
        return score;
    }
}