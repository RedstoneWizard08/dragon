package common

import "io/fs"

var ViewsFS fs.FS

type ViewMetadata struct {
	Scripts []string `json:"scripts"`
	Styles  []string `json:"styles"`
}
