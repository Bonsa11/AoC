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

	log.SetFlags(log.Ldate | log.Ldate)

	data, err := parse(fileName)
	if err != nil {
		log.Fatal(err)
	}

	sumOfCalibrationValues, err := run(*data)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(sumOfCalibrationValues)
}

func parse(fileName string) (data *Data, err error) {
	// read in file from os into buffer
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

func replaceUsingMap(line string, replacements map[string]string) (newline string) {
	newline = ""

	// set up tweo pointers
	lp := 0
	rp := lp + 1

	for rp <= len(line) {

		// edge case first elem is a digit
		if unicode.IsDigit(rune(line[lp])) {
			fmt.Printf("lp is digit %s \n", string(line[lp]))
			newline += string(line[lp])
			lp = rp
			rp += 1

		} else {
			// check if digit
			newVal := line[lp:rp]
			fmt.Printf("newVal is %v \n", newVal)
			for idx := range newVal {
				valToCheck := newVal[idx:]
				fmt.Printf("value to check is %v \n", valToCheck)
				value, exists := replacements[valToCheck]
				if exists {
					fmt.Print(" !!found value!! \n")
					newline += value
					lp = rp
					break
				}
			}
		}
		rp += 1
	}
	return newline
}

func run(data Data) (sumOfCalibrationValues int, err error) {
	sumOfCalibrationValues = 0

	numberLookup := map[string]string{
		"one":   "1",
		"two":   "2",
		"three": "3",
		"four":  "4",
		"five":  "5",
		"six":   "6",
		"seven": "7",
		"eight": "8",
		"nine":  "9",
	}

	for idx, line := range data.Lines {
		var left, right string

		newline := replaceUsingMap(line, numberLookup)
		fmt.Printf("    Newline for line %v is %v \n", line, newline)

		for _, char := range newline {
			if unicode.IsDigit(char) {
				left = string(char)
				break
			}
		}

		for i := len(newline) - 1; i >= 0; i-- {
			if unicode.IsDigit(rune(newline[i])) {
				right = string(newline[i])
				break
			}
		}
		value, err := strconv.Atoi(left + right)
		if err != nil {
			return -1, fmt.Errorf("unable to parse value from %s, %s: %w", left, right, err)
		}
		fmt.Printf("        Value for line %v is %v \n", idx, value)

		sumOfCalibrationValues += value
	}

	return sumOfCalibrationValues, nil
}