/*
 * Advent of Code 2023 Day 10
 */
package com.java.aoc23;

import java.util.Arrays;
import java.util.ArrayList;
import java.awt.Polygon;

public class Day10 implements Day {
    public void part1(String input) {
        Puzzle puzzle = new Puzzle(input, "E");
        int pathLength = puzzle.solve();
        System.out.println("Part 1: " + pathLength / 2);
    }

    public void part2(String input) {
        Puzzle puzzle = new Puzzle(input, "E");
        puzzle.solve();
        int numberOfPointsInPolygon = puzzle.numberOfPointsInPolygon();
        System.out.println("Part 2: " + numberOfPointsInPolygon);
    }
}

final class Puzzle {
    String[][] input;
    ArrayList<int[]> visitedLocations = new ArrayList<int[]>();
    Polygon polygon;
    int[] startLocation;
    int[] currentLocation;
    String currentDirection; // N, E, S, W
    boolean isFinished = false;
    int pathLength = 0;
    int pointsInPolygon = 0;

    public Puzzle(String input, String startingDirection) {
        this.input = Arrays.stream(input.split("\n"))
                .map(line -> line.split(""))
                .toArray(String[][]::new);
        findStartLocation();
        this.currentLocation = startLocation;
        this.currentDirection = startingDirection;
    }

    public int solve() {
        while (!isFinished) {
            move();
        }
        polygon = new Polygon(visitedLocations.stream().mapToInt(i -> i[0]).toArray(),
                visitedLocations.stream().mapToInt(i -> i[1]).toArray(),
                visitedLocations.size());
        return pathLength;
    }

    public int numberOfPointsInPolygon() {
        for (int i = 0; i < input.length; i++) {
            String[] row = input[i];
            for (int j = 0; j < row.length; j++) {
                if (isPointInPolygon(i, j)) {
                    pointsInPolygon++;
                }
            }
        }
        return pointsInPolygon - (pathLength / 2) + 1;
    }

    private boolean isPointInPolygon(int x, int y) {
        return polygon.contains(x, y);
    }

    private void findStartLocation() {
        for (int i = 0; i < input.length; i++) {
            String[] row = input[i];
            for (int j = 0; j < row.length; j++) {
                String element = row[j];
                if (element.equals("S")) {
                    startLocation = new int[] { i, j };
                    return;
                }
            }
        }
    }

    private void move() {
        String currentElement = input[currentLocation[0]][currentLocation[1]];
        visitedLocations.add(new int[] { currentLocation[0], currentLocation[1] });
        if (currentElement.equals("|")) {
            pathLength++;
            if (currentDirection.equals("N")) {
                currentLocation[0]--;
            } else if (currentDirection.equals("S")) {
                currentLocation[0]++;
            }
        } else if (currentElement.equals("-")) {
            pathLength++;
            if (currentDirection.equals("E")) {
                currentLocation[1]++;
            } else if (currentDirection.equals("W")) {
                currentLocation[1]--;
            }
        } else if (currentElement.equals("L")) {
            pathLength++;
            if (currentDirection.equals("S")) {
                currentLocation[1]++;
                currentDirection = "E";
            } else if (currentDirection.equals("W")) {
                currentLocation[0]--;
                currentDirection = "N";
            }
        } else if (currentElement.equals("J")) {
            pathLength++;
            if (currentDirection.equals("S")) {
                currentLocation[1]--;
                currentDirection = "W";
            } else if (currentDirection.equals("E")) {
                currentLocation[0]--;
                currentDirection = "N";
            }
        } else if (currentElement.equals("7")) {
            pathLength++;
            if (currentDirection.equals("N")) {
                currentLocation[1]--;
                currentDirection = "W";
            } else if (currentDirection.equals("E")) {
                currentLocation[0]++;
                currentDirection = "S";
            }
        } else if (currentElement.equals("F")) {
            pathLength++;
            if (currentDirection.equals("N")) {
                currentLocation[1]++;
                currentDirection = "E";
            } else if (currentDirection.equals("W")) {
                currentLocation[0]++;
                currentDirection = "S";
            }
        } else if (currentElement.equals(".")) {
            isFinished = true;
        } else if (currentElement.equals("S")) {
            if (pathLength > 0) {
                isFinished = true;
            } else {
                pathLength++;
                if (currentDirection.equals("N")) {
                    currentLocation[0]--;
                } else if (currentDirection.equals("S")) {
                    currentLocation[0]++;
                } else if (currentDirection.equals("E")) {
                    currentLocation[1]++;
                } else if (currentDirection.equals("W")) {
                    currentLocation[1]--;
                }
            }
        } else {
            System.out.println("Unknown element: " + currentElement);
        }
    }
}