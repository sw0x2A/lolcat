%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: lolcat
Summary: lolcat concatenates files or standard input to standard output. With rainbows.

Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: Unlicense OR MIT
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://github.com/sw0x2A/lolcat

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
