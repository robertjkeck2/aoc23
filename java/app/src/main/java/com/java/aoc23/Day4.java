/*
 * Advent of Code 2023 Day 4
 */
package com.java.aoc23;

import java.util.List;
import java.util.ArrayList;

public class Day4 implements Day {
    public void part1(String input) {
        String[] cardValues = input.split("\n");
        int sum = 0;
        for (String cardValue : cardValues) {
            Card card = new Card(cardValue, new Score(0));
            sum += card.calculateScore();
        }
        System.out.println("Part 1: " + sum);
    }

    public void part2(String input) {
        String[] cardValues = input.split("\n");
        Score score = new Score(0);
        for (int i = 0; i < cardValues.length; i++) {
            Card card = new Card(cardValues[i], score);
            if (card.getNumMatches() > 0) {
                spawnCards(cardValues, i, card.getNumMatches(), score);
            }
        }
        System.out.println("Part 2: " + score.score);
    }

    public void spawnCards(String[] cardValues, int row, int numMatches, Score score) {
        for (int i = 1; i <= numMatches; i++) {
            Card card = new Card(cardValues[row + i], score);
            if (card.getNumMatches() > 0) {
                spawnCards(cardValues, row + i, card.getNumMatches(), score);
            }
        }
    }
}

final class Score {
    public int score;

    public Score(int score) {
        this.score = score;
    }

    public void increment() {
        this.score += 1;
    }
}

final class Card {
    public final int cardNum;
    public final int[] winningNums;
    public final int[] playerNums;

    public Card(String input, Score score) {
        score.increment();

        String[] cardValues = input.split(": ");
        String cardNumString = cardValues[0];
        this.cardNum = Integer.parseInt(cardNumString.split("\s+")[1]);

        String gameNums = cardValues[1];
        String[] gameNumsSplit = gameNums.split("\\s+\\|\\s+");
        String[] winningNumsString = gameNumsSplit[0].trim().split("\s+");
        String[] playerNumsString = gameNumsSplit[1].trim().split("\s+");
        this.winningNums = new int[winningNumsString.length];
        for (int i = 0; i < winningNumsString.length; i++) {
            this.winningNums[i] = Integer.parseInt(winningNumsString[i]);
        }
        this.playerNums = new int[playerNumsString.length];
        for (int i = 0; i < playerNumsString.length; i++) {
            this.playerNums[i] = Integer.parseInt(playerNumsString[i]);
        }
    }

    public int calculateScore() {
        int score = 0;
        List<Integer> winningNumsList = new ArrayList<>();
        for (int num : winningNums) {
            winningNumsList.add(num);
        }
        for (int i = 0; i < playerNums.length; i++) {
            if (winningNumsList.contains(playerNums[i])) {
                if (score == 0) {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        return score;
    }

    public int getNumMatches() {
        int matches = 0;
        List<Integer> winningNumsList = new ArrayList<>();
        for (int num : winningNums) {
            winningNumsList.add(num);
        }
        for (int i = 0; i < playerNums.length; i++) {
            if (winningNumsList.contains(playerNums[i])) {
                matches += 1;
            }
        }
        return matches;
    }
}