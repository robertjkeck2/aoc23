/*
 * Advent of Code 2023 Day 8
 */
package com.java.aoc23;

import java.util.HashMap;

public class Day8 implements Day {
    public void part1(String input) {
        String[] lines = input.split("\n");
        String[] instructions = lines[0].split("");
        HashMap<String, String> map = new HashMap<String, String>();
        for (int i = 0; i < lines.length; i++) {
            if (i == 0 || i == 1) {
                continue;
            }
            String[] line = lines[i].split(" = ");
            String key = line[0];
            String[] value = line[1].replace("(", "").replace(")", "").split(", ");

            map.put(key + "-L", value[0]);
            map.put(key + "-R", value[1]);
        }
        boolean found = false;
        long index = 0;
        String location = "AAA";
        while (!found) {
            String instruction = instructions[(int) (index % instructions.length)];
            String newLocation;
            if (instruction.equals("L")) {
                newLocation = map.get(location + "-L");
            } else {
                newLocation = map.get(location + "-R");
            }
            index++;
            if (newLocation.equals("ZZZ")) {
                found = true;
                break;
            }
            location = newLocation;
        }
        System.out.println("Part 1: " + index);
    }

    public void part2(String input) {
        String[] lines = input.split("\n");
        String[] instructions = lines[0].split("");
        HashMap<String, String> map = new HashMap<String, String>();
        for (int i = 0; i < lines.length; i++) {
            if (i == 0 || i == 1) {
                continue;
            }
            String[] line = lines[i].split(" = ");
            String key = line[0];
            String[] value = line[1].replace("(", "").replace(")", "").split(", ");

            map.put(key + "-L", value[0]);
            map.put(key + "-R", value[1]);
        }
        String[] locations = { "AAA", "HVA", "FXA", "LBA", "PSA", "GHA" };
        long lcm = 1;
        long[] pathLength = new long[locations.length];
        for (int i = 0; i < locations.length; i++) {
            boolean found = false;
            long index = 0;
            String location = locations[i];
            while (!found) {
                String instruction = instructions[(int) (index % instructions.length)];
                String newLocation;
                if (instruction.equals("L")) {
                    newLocation = map.get(location + "-L");
                } else {
                    newLocation = map.get(location + "-R");
                }
                index++;
                if (newLocation.endsWith("Z")) {
                    found = true;
                    break;
                }
                location = newLocation;
            }
            pathLength[i] = index;
        }
        // Find LCM of path lengths
        for (int i = 0; i < pathLength.length; i++) {
            lcm = lcm * pathLength[i] / gcd(lcm, pathLength[i]);
        }
        System.out.println("Part 2: " + lcm);
    }

    public long gcd(long a, long b) {
        if (b == 0)
            return a;
        return gcd(b, a % b);
    }
}