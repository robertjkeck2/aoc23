/*
 * Advent of Code 2023 Day 1
 */
package com.java.aoc23;

import java.util.ArrayList;

public class Day1 implements Day {
    public void part1(String input) {
        String[] calibration_values = input.split("\n");
        int sum = 0;
        for (String calibration_value : calibration_values) {
            ArrayList<String> numbers = new ArrayList<String>();
            for (int i = 0; i < calibration_value.length(); i++) {
                if (Character.isDigit(calibration_value.charAt(i))) {
                    numbers.add(calibration_value.substring(i, i + 1));
                }
            }
            int number = Integer.parseInt(numbers.get(0)) * 10 + Integer.parseInt(numbers.get(numbers.size() - 1));
            sum += number;
        }
        System.out.println("Part 1: " + sum);
    }

    public void part2(String input) {
        System.out.println("Part 2 not implemented yet");
    }
}