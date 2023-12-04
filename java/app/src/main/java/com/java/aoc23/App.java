/*
 * Advent of Code 2023
 */
package com.java.aoc23;

public class App {
    public static void main(String[] args) {
        String day_num = args[0];
        String part_num = args[1];
        String input = InputReader.read_file_input(day_num);
        Day day = new Day1();
        switch (day_num) {
            case "1":
                day = new Day1();
                break;
            default:
                break;
        }
        switch (part_num) {
            case "1":
                day.part1(input);
                break;
            case "2":
                day.part2(input);
                break;
            default:
                break;
        }
    }
}
