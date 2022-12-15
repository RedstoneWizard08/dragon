package sass

import (
	"errors"
	"fmt"
	"os/exec"
	"strings"
)

func (compiler *SassCompiler) BuildCommandArgs(files map[string]string) string {
	buildEnd := ""

	basicOpts := GetDefaultArguments()
	ioOpts := compiler.IOOptions()
	sourceMapOpts := compiler.SourceMapOptions()
	watchOpts := compiler.WatchOptions()

	for key, value := range files {
		buildEnd += fmt.Sprintf("%s:%s ", key, value)
	}

	opts := fmt.Sprintf("%s%s%s%s%s", basicOpts, ioOpts, sourceMapOpts, watchOpts, buildEnd)

	return opts
}

func (compiler *SassCompiler) GetBuildCmd(files map[string]string) (string, []string, error) {
	bin, err := GetBinary()

	if err != nil {
		return "", []string{}, err
	}

	args := strings.Split(compiler.BuildCommandArgs(files), " ")

	return bin, args, nil
}

func (compiler *SassCompiler) Compile(files map[string]string) error {
	bin, args, err := compiler.GetBuildCmd(files)

	if err != nil {
		return err
	}

	args = args[:len(args)-1]
	cmd := exec.Command(bin, args...)

	err = cmd.Run()

	if err != nil {
		msg := err.Error()

		message := fmt.Sprintf("Unable to run command: \n%s %s\nError: %s", bin, args, msg)

		return errors.New(message)
	}

	return nil
}
