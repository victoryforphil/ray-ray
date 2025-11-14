"""Test runner for ray_ray tests."""
import sys
import pytest

if __name__ == "__main__":
    # Run pytest with the arguments passed to the script
    sys.exit(pytest.main(sys.argv[1:]))
