from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup_requires = ['setuptools-rust>=0.10.1']
install_requires = []

setup(
    name="libosu",
    version="0.0.5",
    packages=["libosu"],
    rust_extensions=[RustExtension("libosu", "Cargo.toml", binding=Binding.PyO3)],
    install_requires=install_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
)
