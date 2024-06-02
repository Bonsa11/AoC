package main

import (
	"testing"
)

const TEST_ANSWER int = 142

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
