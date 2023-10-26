package routes

import (
	"fmt"
	"strconv"

	"github.com/RedstoneWizard08/dragon/pkg/logger"
	fws "github.com/gofiber/contrib/websocket"
	"github.com/gofiber/fiber/v2"
	"github.com/gorilla/websocket"
)

const port int = 4001
const host string = "127.0.0.1"

func WsProxy() func(ctx *fiber.Ctx) error {
	return func(fctx *fiber.Ctx) error {
		return fws.New(func(ctx *fws.Conn) {
			log := logger.New("WEBSOCKET")

			log.Info("Connecting to HMR socket...")

			c, _, err := websocket.DefaultDialer.Dial("ws://"+host+":"+strconv.Itoa(port)+"/_next/webpack-hmr", nil)

			if err != nil {
				log.Error(fmt.Sprintf("Connection error: %s", err))
				return
			}

			log.Info("Connected!")

			defer c.Close()

			var mt int
			var msg []byte

			for {
				if mt, msg, err = c.ReadMessage(); err != nil {
					log.Error(fmt.Sprintf("Read: %s", err))
					break
				}

				if err = ctx.WriteMessage(mt, msg); err != nil {
					log.Error(fmt.Sprintf("Write: %s", err))
					break
				}
			}
		})(fctx)
	}
}
