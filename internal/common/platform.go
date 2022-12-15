package common

import (
	"os"
	"runtime"
)

type Arch int

const (
	AMD64 Arch = 0
	I386  Arch = 1
	ARM64 Arch = 2
	ARM   Arch = 3
)

func (a Arch) String() string {
	switch a {
	case AMD64:
		return "amd64"
	case I386:
		return "i386"
	case ARM64:
		return "arm64"
	case ARM:
		return "arm"
	}

	return "unknown"
}

func GetArch() Arch {
	goarch := runtime.GOARCH

	switch goarch {
	case "amd64":
		return AMD64
	case "386":
		return I386
	case "arm64":
		return ARM64
	case "arm":
		return ARM
	}

	return -1
}

func GetWorkingDirectory() string {
	wd, err := os.Getwd()

	if err != nil {
		return "./"
	}

	return wd
}
