// Package substrait provides access to Substrait artifacts via embed.FS.
// Use substrait.GetSubstraitFS() to retrieve the embed.FS object.
package substrait

import "embed"

// Add all directories which should be exposed in below
//
//go:embed extensions/*
//go:embed tests/cases/*/*
var substraitFS embed.FS

func GetSubstraitFS() embed.FS {
	return substraitFS
}
