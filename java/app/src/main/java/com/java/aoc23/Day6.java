/*
 * Advent of Code 2023 Day 6
 */
package com.java.aoc23;

import java.util.ArrayList;
import java.util.Arrays;

public class Day6 implements Day {
    public void part1(String input) {
        Race[] races = initializeRaces(input);
        long marginOfError = 1;
        for (Race race : races) {
            marginOfError *= race.getWaitTimesOverRecordFast();
        }
        System.out.println("Part 1: " + marginOfError);
    }

    public void part2(String input) {
        Race race = initializeBigRace(input);
        long marginOfError = race.getWaitTimesOverRecordFast();
        System.out.println("Part 2: " + marginOfError);
    }

    public Race[] initializeRaces(String input) {
        String[] inputSplit = input.split("\n");
        String[] times = inputSplit[0].split("\s+");
        String[] distances = inputSplit[1].split("\s+");
        Race[] races = new Race[times.length - 1];
        for (int i = 1; i < times.length; i++) {
            races[i - 1] = new Race(Long.parseLong(times[i]), Long.parseLong(distances[i]));
        }
        return races;
    }

    public Race initializeBigRace(String input) {
        String[] inputSplit = input.split("\n");
        String[] times = inputSplit[0].split("\s+");
        String[] distances = inputSplit[1].split("\s+");
        long time = Long.parseLong(String.join("", Arrays.copyOfRange(times, 1, times.length)));
        long distance = Long.parseLong(String.join("", Arrays.copyOfRange(distances, 1, distances.length)));
        Race race = new Race(time, distance);
        return race;
    }
}

final class Race {
    public long time;
    public long recordDistance;

    public Race(long time, long recordDistance) {
        this.time = time;
        this.recordDistance = recordDistance;
    }

    public long getDistance(long waitTime) {
        return (this.time - waitTime) * waitTime;
    }

    public long getWaitTimesOverRecord() {
        ArrayList<Long> waitTimes = new ArrayList<Long>();
        for (long i = 0; i < this.time; i++) {
            if (this.getDistance(i) > this.recordDistance) {
                waitTimes.add(i);
            }
        }
        return waitTimes.size();
    }

    public long getWaitTimesOverRecordFast() {
        long first = 0;
        while (true) {
            if (this.getDistance(first) > this.recordDistance) {
                break;
            }
            first++;
        }
        return this.time - first - first + 1;
    }
}