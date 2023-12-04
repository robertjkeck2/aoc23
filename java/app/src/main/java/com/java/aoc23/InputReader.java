/*
 * InputReader reads the input file and returns the input as a strings
 */
package com.java.aoc23;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

public class InputReader {
    public static String read_file_input(String day_num) {
        String filepath = "src/main/java/com/java/aoc23/data/day" + day_num + "/input.txt";
        Path path = Paths.get(filepath);
        if (!Files.exists(path)) {
            System.out.println("File not found: " + filepath);
            System.exit(1);
        }
        System.out.println("Reading input from " + filepath);
        String contents;
        try {
            contents = Files.readString(path, StandardCharsets.UTF_8);
        } catch (IOException e) {
            contents = "";
        }
        return contents;
    }
}
