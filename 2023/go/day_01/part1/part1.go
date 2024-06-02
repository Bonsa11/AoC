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

func run(data Data) (sumOfCalibrationValues int, err error) {
	sumOfCalibrationValues = 0
	for idx, line := range data.Lines {
		var left, right string

		for _, char := range line {
			if unicode.IsDigit(char) {
				left = string(char)
				break
			}
		}

		for i := len(line) - 1; i >= 0; i-- {
			if unicode.IsDigit(rune(line[i])) {
				right = string(line[i])
				break
			}
		}
		value, err := strconv.Atoi(left + right)
		if err != nil {
			return -1, fmt.Errorf("unable to parse value from %s, %s: %w", left, right, err)
		}
		fmt.Printf("Value for line %v is %v \n", idx, value)

		sumOfCalibrationValues += value
	}

	return sumOfCalibrationValues, nil
}
