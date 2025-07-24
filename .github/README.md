# GitHub Workflows Documentation

This directory contains comprehensive GitHub Actions workflows for automated repository management and optimization.

## 📋 Workflow Overview

### Core CI/CD Workflows

#### 🔄 CI/CD Pipeline (`ci.yml`)
**Triggers**: Push to main/develop, PRs, manual dispatch

**Features**:
- Multi-stage quality checks (formatting, linting, security)
- Comprehensive testing suite with doctests
- Release builds with artifact storage
- Integration testing with server verification
- Performance benchmarking on main branch
- Dependency vulnerability scanning
- Smart caching for faster builds

**Jobs**:
1. **Code Quality** - Formatting, clippy, security audit
2. **Testing** - Unit tests, doctests, multiple Rust versions
3. **Build** - Release builds with artifacts
4. **Integration** - Server startup and port binding tests
5. **Benchmarks** - Performance tracking (main branch only)
6. **Dependency Review** - Security scanning for PRs

#### 🔒 Security Scanning (`security.yml`)
**Triggers**: Push, PRs, weekly schedule, manual dispatch

**Features**:
- Comprehensive security auditing with cargo-audit
- Dependency vulnerability scanning with Trivy
- CodeQL static analysis for Rust
- License compliance checking
- Secret scanning with TruffleHog
- Supply chain security with cargo-deny

**Jobs**:
1. **Security Audit** - Rust vulnerability database
2. **Dependency Scanning** - SARIF upload for GitHub Security
3. **Code Scanning** - CodeQL analysis
4. **License Check** - Compliance verification
5. **Secrets Scanning** - Credential detection
6. **Supply Chain** - Comprehensive dependency analysis

#### 🚀 Release Automation (`release.yml`)
**Triggers**: Version tags, manual dispatch

**Features**:
- Multi-architecture builds (x86_64, aarch64)
- Automated changelog generation
- Security validation before release
- GitHub release creation with assets
- Homebrew formula updates
- Post-release task automation

**Jobs**:
1. **Validate Release** - Version consistency and testing
2. **Build Release** - Multi-target binary builds
3. **Security Check** - Pre-release security validation
4. **Create Release** - GitHub release with changelog
5. **Post-release** - Follow-up task creation
6. **Homebrew Update** - Package manager integration

### Quality Assurance Workflows

#### 🎯 Quality Gates (`quality-gates.yml`)
**Triggers**: Push, PRs, daily schedule, manual dispatch

**Features**:
- Code coverage analysis with LLVM
- Mutation testing for test quality
- Performance benchmarking with Criterion
- Memory safety analysis with Miri
- Documentation quality checks
- Advanced static analysis

**Jobs**:
1. **Code Coverage** - LLVM coverage with Codecov upload
2. **Mutation Testing** - Test effectiveness validation
3. **Performance Testing** - Benchmark execution
4. **Memory Safety** - Miri and AddressSanitizer
5. **Documentation** - Coverage and link checking
6. **Static Analysis** - Advanced clippy and tool analysis

#### 🤖 PR Automation (`pr-automation.yml`)
**Triggers**: PR events, reviews, comments

**Features**:
- Semantic PR title validation
- Automatic labeling and size detection
- Reviewer assignment based on file changes
- Merge conflict detection
- Draft PR handling
- Dependabot auto-merge for safe updates

**Jobs**:
1. **PR Validation** - Title and size checking
2. **Auto-assign** - Reviewer assignment
3. **Auto-label** - Content-based labeling
4. **Conflict Detection** - Merge conflict alerts
5. **PR Checklist** - Automated reminder comments
6. **Auto-merge** - Dependabot patch/minor updates

### Monitoring & Management Workflows

#### 📊 Repository Monitoring (`monitoring.yml`)
**Triggers**: Weekly/daily schedule, push to main, manual dispatch

**Features**:
- Repository health scoring
- Dependency update tracking
- Workflow performance analysis
- Stale issue detection
- Security advisory monitoring

**Jobs**:
1. **Repository Health** - Comprehensive metrics
2. **Dependency Updates** - Outdated package detection
3. **Workflow Performance** - Success rates and timing
4. **Stale Detection** - Issue aging analysis
5. **Security Monitoring** - Advisory and alert tracking

#### 🎫 Issue Management (`issue-management.yml`)
**Triggers**: Issue events, schedule, manual dispatch

**Features**:
- Intelligent auto-triaging with AI-like labeling
- New contributor welcome automation
- Maintainer assignment based on expertise
- Follow-up reminders and stale issue handling
- Project metrics collection and reporting

**Jobs**:
1. **Auto-triage** - Smart labeling and assignment
2. **Issue Reminder** - Follow-up automation
3. **Project Metrics** - Statistical analysis
4. **Cleanup** - Label management for closed issues
5. **Community Stats** - Contributor activity tracking

## 🔧 Configuration Files

### Required Secrets
```yaml
# Repository Settings > Secrets and Variables > Actions
CODECOV_TOKEN: # For code coverage uploads
GITHUB_TOKEN: # Automatically provided
```

### Branch Protection Rules
Recommended settings for `main` branch:
- Require PR reviews (1+ reviewers)
- Require status checks to pass
- Require up-to-date branches
- Include administrators
- Allow force pushes: false
- Allow deletions: false

### Repository Settings
- Issues: Enabled ✅
- Projects: Enabled ✅  
- Wiki: Enabled ✅
- Security: All features enabled ✅
- Dependency graph: Enabled ✅
- Dependabot alerts: Enabled ✅

## 📈 Workflow Triggers Summary

| Workflow | Push | PR | Schedule | Manual | Tags |
|----------|------|----|---------|---------|----- |
| CI/CD Pipeline | ✅ | ✅ | ❌ | ✅ | ❌ |
| Security Scanning | ✅ | ✅ | Weekly | ✅ | ❌ |
| Release Automation | ❌ | ❌ | ❌ | ✅ | ✅ |
| Quality Gates | ✅ | ✅ | Daily | ✅ | ❌ |
| PR Automation | ❌ | ✅ | ❌ | ❌ | ❌ |
| Monitoring | Main only | ❌ | Weekly/Daily | ✅ | ❌ |
| Issue Management | ❌ | ❌ | Weekly | ✅ | ❌ |

## 🏷️ Automated Labels

### Priority Labels
- `priority: low` - Minor improvements, documentation
- `priority: medium` - Standard features and bugs  
- `priority: high` - Critical bugs, urgent features

### Type Labels
- `type: bug` - Bug reports and fixes
- `type: enhancement` - New features and improvements
- `type: documentation` - Documentation updates
- `type: question` - Support questions
- `type: performance` - Performance-related issues

### Component Labels
- `component: build` - Build system and dependencies
- `component: ui` - User interface and GUI
- `component: server` - HTTP server and networking
- `component: events` - Event processing system

### Status Labels
- `status: draft` - Draft pull requests
- `status: ready-for-review` - Ready for maintainer review
- `status: stale` - Issues inactive for 90+ days
- `status: resolved` - Closed bug reports

### Special Labels
- `good first issue` - Beginner-friendly issues
- `help wanted` - Community contribution welcome
- `needs: maintainer-response` - Waiting for maintainer
- `needs: info` - Waiting for user information

## 🚨 Security Features

### Vulnerability Management
- Automated security audits with `cargo-audit`
- Dependency scanning with Trivy
- CodeQL static analysis
- Secret detection with TruffleHog
- Supply chain validation with `cargo-deny`

### License Compliance
- Automatic license checking
- Problematic license detection (GPL, AGPL)
- License report generation
- Compliance enforcement in CI

### Dependency Security
- Dependabot integration
- Automated vulnerability alerts
- Safe update automation for patches
- Manual review for major updates

## 📊 Performance Monitoring

### Code Coverage
- LLVM-based coverage analysis
- Codecov integration for reporting
- Coverage thresholds and trends
- HTML report generation

### Benchmarking
- Criterion.rs integration
- Performance regression detection
- Historical trend analysis
- Automated alerts for degradation

### Quality Metrics
- Mutation testing for test quality
- Static analysis with advanced clippy
- Documentation coverage tracking
- Code complexity analysis

## 🔄 Release Process

### Automated Release Workflow
1. **Version Tag**: Create tag matching `v*` pattern
2. **Validation**: Version consistency and security checks
3. **Build**: Multi-architecture release binaries
4. **Release**: GitHub release with changelog
5. **Distribution**: Package manager updates
6. **Follow-up**: Post-release task creation

### Manual Release Process
1. Use `workflow_dispatch` with version input
2. Specify prerelease flag if needed
3. Follow automated validation and build
4. Review and approve release assets

## 🛠️ Maintenance

### Regular Tasks
- Weekly repository health review
- Monthly dependency updates
- Quarterly workflow optimization
- Security advisory monitoring

### Workflow Updates
- Monitor GitHub Actions marketplace for updates
- Review and update action versions quarterly
- Test workflow changes in feature branches
- Document any breaking changes

## 📚 Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI/CD Best Practices](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
- [Security Best Practices](https://docs.github.com/en/code-security)
- [Branch Protection Rules](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/about-protected-branches)

---

*This documentation is automatically maintained and updated with workflow changes.*