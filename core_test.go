package substrait

import (
	"embed"
	"io/fs"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestGetSubstraitFS(t *testing.T) {
	got := GetSubstraitFS()
	filePaths, err := ListFiles(got, ".")
	require.NoError(t, err)
	assert.Greater(t, len(filePaths), 19)
	assert.Contains(t, filePaths, "tests/cases/arithmetic/add.test")
	assert.Contains(t, filePaths, "tests/cases/arithmetic/max.test")
	assert.Contains(t, filePaths, "tests/cases/arithmetic_decimal/power.test")
	assert.Contains(t, filePaths, "tests/cases/datetime/lt_datetime.test")

	assert.Contains(t, filePaths, "extensions/functions_arithmetic.yaml")
	assert.Contains(t, filePaths, "extensions/functions_arithmetic_decimal.yaml")
	assert.Contains(t, filePaths, "extensions/functions_datetime.yaml")
}

func ListFiles(embedFs embed.FS, root string) ([]string, error) {
	var files []string
	err := fs.WalkDir(embedFs, root, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() {
			files = append(files, path)
		}
		return nil
	})
	return files, err
}
