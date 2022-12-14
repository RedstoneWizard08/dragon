package logger

import (
	"fmt"

	"github.com/fatih/color"
)

func get_prefixes() map[string](string) {
	prefixes := make(map[string](string))

	info_color := color.New(color.BgBlue, color.FgBlack, color.Bold).SprintFunc()
	warn_color := color.New(color.BgYellow, color.FgBlack, color.Bold).SprintFunc()
	error_color := color.New(color.BgRed, color.FgBlack, color.Bold).SprintFunc()
	boot_color := color.New(color.BgHiMagenta, color.FgBlack, color.Bold).SprintFunc()

	prefixes["info"] = info_color(" INFO ")
	prefixes["warn"] = warn_color(" WARN ")
	prefixes["error"] = error_color(" ERROR ")
	prefixes["boot"] = boot_color(" BOOT ")

	return prefixes
}

type Logger struct {
	Name string
}

func New(Name string) Logger {
	return Logger{
		Name,
	}
}

func (logger *Logger) Info(msg string) {
	color.Unset()

	info_prefix := get_prefixes()["info"]
	module_color := color.New(color.FgCyan, color.Bold).SprintFunc()

	fmt.Printf("[%s] %s %s\n", module_color(logger.Name), info_prefix, msg)

	color.Unset()
}

func (logger *Logger) Warn(msg string) {
	color.Unset()

	warn_prefix := get_prefixes()["warn"]
	module_color := color.New(color.FgCyan, color.Bold).SprintFunc()

	fmt.Printf("[%s] %s %s\n", module_color(logger.Name), warn_prefix, msg)

	color.Unset()
}

func (logger *Logger) Error(msg string) {
	color.Unset()

	error_prefix := get_prefixes()["error"]
	module_color := color.New(color.FgCyan, color.Bold).SprintFunc()

	fmt.Printf("[%s] %s %s\n", module_color(logger.Name), error_prefix, msg)

	color.Unset()
}

func (logger *Logger) Boot(msg string) {
	color.Unset()

	boot_prefix := get_prefixes()["boot"]
	module_color := color.New(color.FgCyan, color.Bold).SprintFunc()

	fmt.Printf("[%s] %s %s\n", module_color(logger.Name), boot_prefix, msg)

	color.Unset()
}
