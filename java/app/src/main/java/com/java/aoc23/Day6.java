/*
 * Advent of Code 2023 Day 6
 */
package com.java.aoc23;

import java.util.ArrayList;

public class Day6 implements Day {
    public void part1(String input) {
        Race[] races = initializeRaces(input);
        int marginOfError = 1;
        for (Race race : races) {
            marginOfError *= race.getWaitTimesOverRecord();
        }
        System.out.println("Part 1: " + marginOfError);
    }

    public void part2(String input) {
        System.out.println("Part 2: " + "TODO");
    }

    public Race[] initializeRaces(String input) {
        String[] inputSplit = input.split("\n");
        String[] times = inputSplit[0].split("\s+");
        String[] distances = inputSplit[1].split("\s+");
        Race[] races = new Race[times.length - 1];
        for (int i = 1; i < times.length; i++) {
            races[i - 1] = new Race(Integer.parseInt(times[i]), Integer.parseInt(distances[i]));
        }
        return races;
    }
}

final class Race {
    public int time;
    public int recordDistance;

    public Race(int time, int recordDistance) {
        this.time = time;
        this.recordDistance = recordDistance;
    }

    public int getDistance(int waitTime) {
        int distance = 0;
        int time = 0;
        int speed = waitTime;
        while (time < this.time - waitTime) {
            time++;
            distance += speed;
        }
        return distance;
    }

    public int getWaitTimesOverRecord() {
        ArrayList<Integer> waitTimes = new ArrayList<Integer>();
        for (int i = 0; i < this.time; i++) {
            if (this.getDistance(i) > this.recordDistance) {
                waitTimes.add(i);
            }
        }
        return waitTimes.size();
    }
}