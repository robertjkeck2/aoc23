/*
 * Advent of Code 2023 Day 1
 */
package com.java.aoc23;

import java.util.ArrayList;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

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
        String[] calibration_values = input.split("\n");
        int sum = 0;
        for (String calibration_value : calibration_values) {
            int[] numbers = parse_digits(calibration_value);
            int number = numbers[0] * 10 + numbers[numbers.length - 1];
            sum += number;
        }
        System.out.println("Part 2: " + sum);
    }

    public int[] parse_digits(String input) {
        Pattern re = Pattern.compile("(?=(one|two|three|four|five|six|seven|eight|nine|\\d))",
                Pattern.CASE_INSENSITIVE);
        Matcher m = re.matcher(input);
        ArrayList<Integer> digits = new ArrayList<Integer>();
        while (m.find()) {
            String digit = m.group(1);
            if (digit.equals("one")) {
                digits.add(1);
            } else if (digit.equals("two")) {
                digits.add(2);
            } else if (digit.equals("three")) {
                digits.add(3);
            } else if (digit.equals("four")) {
                digits.add(4);
            } else if (digit.equals("five")) {
                digits.add(5);
            } else if (digit.equals("six")) {
                digits.add(6);
            } else if (digit.equals("seven")) {
                digits.add(7);
            } else if (digit.equals("eight")) {
                digits.add(8);
            } else if (digit.equals("nine")) {
                digits.add(9);
            } else {
                digits.add(Integer.parseInt(digit));
            }
        }
        int[] numbers = new int[digits.size()];
        for (int i = 0; i < digits.size(); i++) {
            numbers[i] = digits.get(i);
        }
        return numbers;
    }
}