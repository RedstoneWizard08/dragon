package sass

import (
	_ "embed"
	"errors"
	"os"
	"path"
	"runtime"

	"github.com/RedstoneWizard08/dragon/internal/common"
)

type BinaryFile []byte

//go:embed binaries/sass.amd64
var BinaryDataAMD64 BinaryFile

//go:embed binaries/sass.i386
var BinaryDataI386 BinaryFile

//go:embed binaries/sass.arm64
var BinaryDataARM64 BinaryFile

//go:embed binaries/sass.arm
var BinaryDataARM BinaryFile

func GetBinaryFile(arch common.Arch) (BinaryFile, error) {
	switch arch {
	case common.AMD64:
		return BinaryDataAMD64, nil

	case common.I386:
		return BinaryDataI386, nil

	case common.ARM64:
		return BinaryDataARM64, nil

	case common.ARM:
		return BinaryDataARM, nil
	}

	return []byte{}, errors.New("Unable to find the SASS binary!")
}

func GetBinaryName(arch common.Arch) (string, error) {
	switch arch {
	case common.AMD64:
		return "sass.amd64", nil

	case common.I386:
		return "sass.i386", nil

	case common.ARM64:
		return "sass.arm64", nil

	case common.ARM:
		return "sass.arm", nil
	}

	return "", errors.New("Unable to find the SASS binary!")
}

func GetBinDirectory() (string, error) {
	if runtime.GOOS == "linux" {
		home, real := os.LookupEnv("HOME")

		if !real {
			return "", errors.New("No HOME environment variable detected!")
		}

		dir := path.Join(home, ".cache", "sass-bin")

		return dir, nil
	}

	return "", errors.New("Unsupported OS!")
}

func GetBinaryPath(arch common.Arch) (string, error) {
	bindir, err := GetBinDirectory()

	if err != nil {
		return "", err
	}

	name, err := GetBinaryName(arch)

	if err != nil {
		return "", err
	}

	binpath := path.Join(bindir, name)

	return binpath, nil
}

func WriteBinary(arch common.Arch) bool {
	binpath, err := GetBinaryPath(arch)

	if err != nil {
		return false
	}

	bindir, err := GetBinDirectory()

	if err != nil {
		return false
	}

	bin, err := GetBinaryFile(arch)

	if err != nil {
		return false
	}

	if _, err := os.Stat(bindir); errors.Is(err, os.ErrNotExist) {
		err = os.MkdirAll(bindir, 0777)

		if err != nil {
			return false
		}
	}

	if _, err := os.Stat(binpath); errors.Is(err, os.ErrNotExist) {
		err = os.WriteFile(binpath, bin, 0777)

		return err == nil
	}

	return true
}

func GetBinary() (string, error) {
	WriteBinary(common.GetArch())

	return GetBinaryPath(common.GetArch())
}
