package main

import (
	"testing"
)

const TEST_ANSWER int = 281

func TestReplaceUsingMap(t *testing.T) {

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

	lines := []string{
		"two1nine",
		"eightwothree",
		"abcone2threexyz",
		"xtwone3four",
		"4nineeightseven2",
		"zoneight234",
		"7pqrstsixteen",
	}

	answers := []string{
		"219",
		"823",
		"123",
		"2134",
		"49872",
		"18234",
		"76",
	}

	for idx, line := range lines {
		val := replaceUsingMap(line, numberLookup)
		if val != answers[idx] {
			t.Fatalf("value %s did not match answer %s", val, answers[idx])
		} else {
			t.Logf("line %d was right", idx)
		}
	}

}

func TestRun(t *testing.T) {
	fileName := "./test.txt"
	data, err := parse(fileName)
	if err != nil {
		t.Fatalf("unable to read in %s", fileName)
	}

	sumOfCalibrationValues, err := run(*data)
	if err != nil {
		t.Fatalf("unable to parse data: %s", err)
	}

	if !(sumOfCalibrationValues == TEST_ANSWER) {
		t.Fatalf("Got wrong answer: %d", sumOfCalibrationValues)
	}
}
