package cmd

import (
	"fmt"
	"os"
	"os/signal"

	"github.com/RedstoneWizard08/dragon/internal/common"
	"github.com/RedstoneWizard08/dragon/internal/middleware"
	"github.com/RedstoneWizard08/dragon/internal/routes"
	"github.com/RedstoneWizard08/dragon/pkg/config"
	"github.com/RedstoneWizard08/dragon/pkg/logger"
	"github.com/gofiber/fiber/v2"
	"github.com/joho/godotenv"
)

func RunCMD(config config.Config) {
	log := logger.New("SERVER")

	err := godotenv.Load()

	if err != nil {
		log.Warn("Error loading .env file!")
	}

	log.Info("Configuring app...")

	app := fiber.New(common.GetFiberConfig())

	log.Info("Setting up app...")

	middleware.SetupMiddleware(app)
	routes.Setup(app)

	log.Info("Preparing to start...")

	address := fmt.Sprintf("%s:%v", config.Host, config.Port)

	log.Boot(fmt.Sprintf("Listening on %s", address))
	log.Error(app.Listen(address).Error())
}

func Exit(config config.Config) {
	code := make(chan os.Signal)
	signal.Notify(code, os.Interrupt)

	go func() {
		sig := <-code

		log := logger.New("SHUTDOWN")

		log.Info(fmt.Sprintf("Got %s signal. Shutting down...", sig))
		log.Info("Saving config...")

		config.Save()

		log.Info("Goodbye!")

		os.Exit(0)
	}()
}
