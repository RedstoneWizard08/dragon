package glue

import (
	"embed"
	"fmt"
	"os"
	"os/exec"
	"os/signal"
	"path"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/RedstoneWizard08/dragon/pkg/logger"
	"github.com/RedstoneWizard08/dragon/pkg/option"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/proxy"
)

const (
	Development Environment = 0
	Production  Environment = 1
)

type Environment int8

type NextDevOptions struct {
	Port int
	Host string
}

type NextGlue struct {
	Env Environment
	FS  embed.FS
	Dir string
	Dev option.Option[NextDevOptions]
}

var DefaultDevOptions NextDevOptions = NextDevOptions{
	Port: 4001,
	Host: "127.0.0.1",
}

func (glue *NextGlue) AsHandler() (func(ctx *fiber.Ctx) error, error) {
	log := logger.New("GLUE")

	if glue.Env == Development {
		workdir := glue.Dir
		nextBin, _ := filepath.Abs(path.Join(workdir, "node_modules", ".bin", "next"))
		devOpts := glue.Dev.UnwrapOr(DefaultDevOptions)
		port := strconv.Itoa(devOpts.Port)
		host := devOpts.Host

		cmd := exec.Command(nextBin, "dev", "--port", port, "--hostname", host)
		cmd.Dir = workdir
		cmd.Stdin = os.Stdin
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr

		log.Info(fmt.Sprintf("Using dev server binary %s...", nextBin))
		log.Info(fmt.Sprintf("Starting dev server at %s:%s...", host, port))

		cmd.Start()

		sigc := make(chan os.Signal, 1)
		signal.Notify(sigc, os.Interrupt)

		go func() {
			sig := <-sigc
			log := logger.New("SHUTDOWN")

			log.Info(fmt.Sprintf("Got %s signal. Killing dev server...\n", sig))

			cmd.Process.Kill()
			cmd.Cancel()
		}()

		return func(ctx *fiber.Ctx) error {
			return proxy.Do(ctx, "http://"+host+":"+port+string(ctx.Request().URI().Path()))
		}, nil
	} else {
		return func(ctx *fiber.Ctx) error {
			path := string(ctx.Request().URI().Path())
			content, err := glue.FS.ReadFile(path)

			if err != nil {
				return err
			}

			extarr := strings.Split(path, ".")
			ext := extarr[len(extarr)-1]
			fmt.Println(content)

			return ctx.Type(ext).Send(content)
		}, nil
	}
}
