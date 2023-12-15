/*
 * Advent of Code 2023 Day 14
 */
package com.java.aoc23;

import java.util.ArrayList;

public class Day14 implements Day {
    public void part1(String input) {
        Dish dish = new Dish(input);
        dish.printColumns();
        System.out.println("Part 1: " + "TODO");
    }

    public void part2(String input) {
        System.out.println("Part 2: " + "TODO");
    }
}

final class Dish {
    private DishColumn[] columns;

    public Dish(String input) {
        String[] rows = input.split("\n");
        columns = new DishColumn[rows[0].length()];

        for (int i = 0; i < rows[0].length(); i++) {
            DishColumn column = new DishColumn(new ArrayList<String>());
            for (int j = 0; j < rows.length; j++) {
                column.addRock(rows[j].split("")[i]);
            }
            columns[i] = column;
        }
    }

    public void printColumns() {
        columns[0].printSortedRocks();
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

    public void sort() {
        if (rocks.size() == 0) {
            return;
        }
        String firstRock = rocks.get(0);
        if (firstRock.equals("O") || firstRock.equals("#")) {
            sortedRocks.add(firstRock);
            rocks.remove(0);
            sort();
        } else {
            int stopNum = 0;
            for (int i = 0; i < rocks.size(); i++) {
                if (rocks.get(i + 1).equals("#")) {
                    stopNum = i + 1;
                    break;
                } else if (rocks.get(i + 1).equals("O")) {
                    sortedRocks.add(rocks.get(i + 1));
                    rocks.set(i + 1, ".");
                    rocks.remove(0);
                    break;
                }
            }
            if (stopNum > 0) {
                for (int i = 0; i < stopNum; i++) {
                    sortedRocks.add(".");
                    rocks.remove(0);
                }
            }
            sort();
        }
    }

    public void printSortedRocks() {
        sort();
        for (String rock : sortedRocks) {
            System.out.print(rock);
        }
    }
}