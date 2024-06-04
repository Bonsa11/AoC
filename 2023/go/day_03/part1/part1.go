package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"unicode"
)

type Data struct {
	Lines []string
}

func main() {
	fileName := "../input.txt"

	log.SetFlags(log.Ldate)

	data, err := parse(fileName)
	if err != nil {
		log.Fatal(err)
	}

	val, err := run(*data)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(val)
}

func parse(fileName string) (data *Data, err error) {
	// Read file into buffer
	buf, err := os.ReadFile(fileName)
	if err != nil {
		return nil, fmt.Errorf("read input file: %w", err)
	}

	d := &Data{}
	lines := strings.Split(string(buf), "\n")
	for _, line := range lines {
		if line == "" {
			continue
		}
		d.Lines = append(d.Lines, line)
	}
	return d, nil
}

// Check if a character is a symbol (any non-digit character except for period)
func isSymbol(r rune) bool {
	return !unicode.IsDigit(r) && r != '.'
}

func run(data Data) (int, error) {
	val := 0
	rows := len(data.Lines)
	cols := len(data.Lines[0])

	for y := 0; y < rows; y++ {
		for x := 0; x < cols; x++ {
			currentChar := rune(data.Lines[y][x])
			if unicode.IsDigit(currentChar) {
				number, endX := extractNumber(data.Lines[y], x)
				if isPartNumber(data.Lines, y, x, endX) {
					val += number
				}
				x = endX // Move x to the end of the current number
			}
		}
	}
	return val, nil
}

// Extracts a number starting at position x in line, returns the number and the position after the number
func extractNumber(line string, x int) (int, int) {
	startX := x
	for x < len(line) && unicode.IsDigit(rune(line[x])) {
		x++
	}
	number, _ := strconv.Atoi(line[startX:x])
	return number, x - 1
}

// Check if the number at position (y, x) with end position endX is a part number
func isPartNumber(lines []string, y, x, endX int) bool {
	// Directions array for 8 possible adjacent positions
	rows := len(lines)
	cols := len(lines[0])

	for i := y - 1; i <= y+1; i++ {
		for j := x - 1; j <= endX+1; j++ {
			if i < 0 || i >= rows || j < 0 || j >= cols {
				continue
			}
			if isSymbol(rune(lines[i][j])) {
				return true
			}
		}
	}
	return false
}
