/*
 * Advent of Code 2023 Day 2
 */
package com.java.aoc23;

import java.util.ArrayList;

public class Day2 implements Day {
    public void part1(String input) {
        final int MAX_RED_CUBES = 12;
        final int MAX_GREEN_CUBES = 13;
        final int MAX_BLUE_CUBES = 14;
        String[] gameValues = input.split("\n");
        int allGames = 0;
        int invalidGames = 0;
        for (String gameValue : gameValues) {
            Game game = new Game(gameValue);
            allGames += game.gameNum;
            for (Turn turn : game.turns) {
                if (turn.greenCubes > MAX_GREEN_CUBES || turn.blueCubes > MAX_BLUE_CUBES
                        || turn.redCubes > MAX_RED_CUBES) {
                    invalidGames += game.gameNum;
                    break;
                }
            }
        }
        System.out.println("Part 1: " + (allGames - invalidGames));
    }

    public void part2(String input) {
        String[] gameValues = input.split("\n");
        int sum = 0;
        for (String gameValue : gameValues) {
            Game game = new Game(gameValue);
            sum += game.cubePower();
        }
        System.out.println("Part 2: " + sum);
    }
}

final class Game {
    int gameNum;
    ArrayList<Turn> turns;

    public Game(String input) {
        parseGameNum(input);
        parseTurns(input);
    }

    public void parseGameNum(String input) {
        String gameNum = input.split(" ")[1];
        this.gameNum = Integer.parseInt(gameNum.substring(0, gameNum.length() - 1));
    }

    public void parseTurns(String input) {
        this.turns = new ArrayList<Turn>();
        String rawTurns = input.split(": ")[1];
        String[] rawTurnsSplit = rawTurns.split("; ");
        for (String rawTurn : rawTurnsSplit) {
            this.turns.add(new Turn(rawTurn));
        }
    }

    public int cubePower() {
        int[] minCubes = minCubes();
        return minCubes[0] * minCubes[1] * minCubes[2];
    }

    public int[] minCubes() {
        int[] minCubes = new int[3];
        minCubes[0] = 0;
        minCubes[1] = 0;
        minCubes[2] = 0;
        for (Turn turn : this.turns) {
            if (turn.greenCubes > minCubes[0]) {
                minCubes[0] = turn.greenCubes;
            }
            if (turn.blueCubes > minCubes[1]) {
                minCubes[1] = turn.blueCubes;
            }
            if (turn.redCubes > minCubes[2]) {
                minCubes[2] = turn.redCubes;
            }
        }
        return minCubes;
    }
}

final class Turn {
    int greenCubes;
    int blueCubes;
    int redCubes;

    public Turn(String rawTurn) {
        String[] rawTurnSplit = rawTurn.split(", ");
        for (String rawTurnSplitPart : rawTurnSplit) {
            String[] rawTurnSplitPartSplit = rawTurnSplitPart.split(" ");
            if (rawTurnSplitPartSplit[1].equals("green")) {
                this.greenCubes = Integer.parseInt(rawTurnSplitPartSplit[0]);
            } else if (rawTurnSplitPartSplit[1].equals("blue")) {
                this.blueCubes = Integer.parseInt(rawTurnSplitPartSplit[0]);
            } else if (rawTurnSplitPartSplit[1].equals("red")) {
                this.redCubes = Integer.parseInt(rawTurnSplitPartSplit[0]);
            }
        }
    }
}