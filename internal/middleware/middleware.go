package middleware

import (
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cache"
)

func SetupMiddleware(app *fiber.App) {
	app.Use(cache.New())
	app.Use(LoggerMiddleware)
}
