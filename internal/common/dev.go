package common

import (
	"os"
	"strings"
)

func IsDev() bool {
	res, val := os.LookupEnv("DRAGON_ENV")

	if !val {
		return true
	} else {
		if strings.ToLower(res) == "production" {
			return false
		} else {
			return true
		}
	}
}
