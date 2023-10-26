package routes

import (
	"github.com/RedstoneWizard08/dragon/pkg/glue"
	"github.com/gofiber/contrib/websocket"
	"github.com/gofiber/fiber/v2"
)

func Setup(app *fiber.App) {
	glue := glue.NextGlue{
		Env: glue.Development,
		Dir: "views",
	}

	handler, err := glue.AsHandler()

	if err != nil {
		panic(err)
	}

	app.Use(func(ctx *fiber.Ctx) error {
		if websocket.IsWebSocketUpgrade(ctx) {
			ctx.Locals("allowed", true)

			return ctx.Next()
		}

		return ctx.Next()
	})

	app.Get("/_next/webpack-hmr", WsProxy())
	app.All("*", handler)
}
