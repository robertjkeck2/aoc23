package com.java.aoc23;

import java.util.ArrayList;

public class Day2 implements Day {
    public void part1(String input) {
        final int MAX_RED_CUBES = 12;
        final int MAX_GREEN_CUBES = 13;
        final int MAX_BLUE_CUBES = 14;
        String[] game_values = input.split("\n");
        int all_games = 0;
        int invalid_games = 0;
        for (String game_value : game_values) {
            Game game = new Game(game_value);
            all_games += game.game_num;
            for (Turn turn : game.turns) {
                if (turn.green_cubes > MAX_GREEN_CUBES || turn.blue_cubes > MAX_BLUE_CUBES
                        || turn.red_cubes > MAX_RED_CUBES) {
                    invalid_games += game.game_num;
                    break;
                }
            }
        }
        System.out.println("Part 1: " + (all_games - invalid_games));
    }

    public void part2(String input) {
        String[] game_values = input.split("\n");
        int sum = 0;
        for (String game_value : game_values) {
            Game game = new Game(game_value);
            sum += game.cube_power();
        }
        System.out.println("Part 2: " + sum);
    }
}

final class Game {
    int game_num;
    ArrayList<Turn> turns;

    public Game(String input) {
        parse_game_num(input);
        parse_turns(input);
    }

    public void parse_game_num(String input) {
        String game_num = input.split(" ")[1];
        this.game_num = Integer.parseInt(game_num.substring(0, game_num.length() - 1));
    }

    public void parse_turns(String input) {
        this.turns = new ArrayList<Turn>();
        String raw_turns = input.split(": ")[1];
        String[] raw_turns_split = raw_turns.split("; ");
        for (String raw_turn : raw_turns_split) {
            this.turns.add(new Turn(raw_turn));
        }
    }

    public int cube_power() {
        int[] min_cubes = min_cubes();
        return min_cubes[0] * min_cubes[1] * min_cubes[2];
    }

    public int[] min_cubes() {
        int[] min_cubes = new int[3];
        min_cubes[0] = 0;
        min_cubes[1] = 0;
        min_cubes[2] = 0;
        for (Turn turn : this.turns) {
            if (turn.green_cubes > min_cubes[0]) {
                min_cubes[0] = turn.green_cubes;
            }
            if (turn.blue_cubes > min_cubes[1]) {
                min_cubes[1] = turn.blue_cubes;
            }
            if (turn.red_cubes > min_cubes[2]) {
                min_cubes[2] = turn.red_cubes;
            }
        }
        return min_cubes;
    }
}

final class Turn {
    int green_cubes;
    int blue_cubes;
    int red_cubes;

    public Turn(String raw_turn) {
        String[] raw_turn_split = raw_turn.split(", ");
        for (String raw_turn_split_part : raw_turn_split) {
            String[] raw_turn_split_part_split = raw_turn_split_part.split(" ");
            if (raw_turn_split_part_split[1].equals("green")) {
                this.green_cubes = Integer.parseInt(raw_turn_split_part_split[0]);
            } else if (raw_turn_split_part_split[1].equals("blue")) {
                this.blue_cubes = Integer.parseInt(raw_turn_split_part_split[0]);
            } else if (raw_turn_split_part_split[1].equals("red")) {
                this.red_cubes = Integer.parseInt(raw_turn_split_part_split[0]);
            }
        }
    }
}