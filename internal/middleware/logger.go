package middleware

import (
	"fmt"

	"github.com/RedstoneWizard08/dragon/pkg/logger"
	"github.com/fatih/color"
	"github.com/gofiber/fiber/v2"
)

func LoggerMiddleware(ctx *fiber.Ctx) error {
	res := ctx.Next()

	log := logger.New("ROUTE")

	path := ctx.Path()
	proto := ctx.Protocol()
	method := ctx.Method()
	remote := ctx.IP()

	status_code := fmt.Sprint(ctx.Response().StatusCode())

	remote_str := fmt.Sprintf(
		"%s%s%s",
		color.BlueString("["),
		color.HiBlueString(remote),
		color.BlueString("]"),
	)

	message := fmt.Sprintf(
		"%s %s %s (Using %s) => %s",
		remote_str,
		color.RedString(method),
		color.GreenString(path),
		color.MagentaString(proto),
		color.YellowString(status_code),
	)

	log.Info(message)

	return res
}
