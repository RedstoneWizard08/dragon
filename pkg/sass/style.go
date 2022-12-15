package sass

func (style SassOutputStyle) String() string {
	switch style {
	case ExpandedStyle:
		return "expanded"
	case CompressedStyle:
		return "compressed"
	}

	return ""
}
