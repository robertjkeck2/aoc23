/*
 * Advent of Code 2023
 */
package com.java.aoc23;

public class App {
    public static void main(String[] args) {
        String dayNum = args[0];
        String partNum = args[1];
        String input = InputReader.readFileInput(dayNum);
        Day day = new Day1();
        switch (dayNum) {
            case "1":
                day = new Day1();
                break;
            case "2":
                day = new Day2();
                break;
            case "3":
                day = new Day3();
                break;
            case "4":
                day = new Day4();
                break;
            case "5":
                day = new Day5();
                break;
            case "6":
                day = new Day6();
                break;
            case "7":
                day = new Day7();
                break;
            case "8":
                day = new Day8();
                break;
            case "9":
                day = new Day9();
                break;
            case "10":
                day = new Day10();
                break;
            case "11":
                day = new Day11();
                break;
            case "12":
                day = new Day12();
                break;
            case "13":
                day = new Day13();
                break;
            case "14":
                day = new Day14();
                break;
            default:
                break;
        }
        switch (partNum) {
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
