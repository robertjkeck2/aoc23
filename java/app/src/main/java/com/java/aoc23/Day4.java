/*
 * Advent of Code 2023 Day 4
 */
package com.java.aoc23;

public class Day4 implements Day {
    public void part1(String input) {
        String[] cardValues = input.split("\n");
        for (String cardValue : cardValues) {
            Card card = new Card(cardValue);
            System.out.println(card.cardNum);
            System.out.println(card.winningNums[0]);
            System.out.println(card.playerNums[0]);
        }
        System.out.println("Part 1: " + "TODO");
    }

    public void part2(String input) {
        System.out.println("Part 2: " + "TODO");
    }
}

final class Card {
    public final int cardNum;
    public final int[] winningNums;
    public final int[] playerNums;

    public Card(String input) {
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
        for (int i = 0; i < this.winningNums.length; i++) {
            score += this.winningNums[i] * (this.winningNums.length - i);
        }
        return score;
    }
}