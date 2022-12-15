package sass

func (linker SourceMapURLLinker) String() string {
	switch linker {
	case AbsoluteLinking:
		return "absolute"
	case RelativeLinking:
		return "relative"
	}

	return ""
}
