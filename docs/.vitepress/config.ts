import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Unifly',
  description: 'CLI + TUI for UniFi Network Controllers',
  base: '/unifly/',
  lastUpdated: true,

  head: [
    ['meta', { name: 'theme-color', content: '#e135ff' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'Unifly Documentation' }],
    ['meta', { property: 'og:description', content: 'CLI + TUI for UniFi Network Controllers' }],
    ['meta', { property: 'og:site_name', content: 'Unifly' }],
    ['meta', { name: 'twitter:card', content: 'summary' }],
  ],

  themeConfig: {
    nav: [
      { text: 'Guide', link: '/guide/' },
      {
        text: 'Reference',
        items: [
          { text: 'CLI Commands', link: '/reference/cli' },
          { text: 'TUI Dashboard', link: '/reference/tui' },
          { text: 'Library API', link: '/reference/library' },
        ]
      },
      { text: 'Architecture', link: '/architecture/' },
      { text: 'Troubleshooting', link: '/troubleshooting' },
    ],

    sidebar: {
      '/guide/': [
        {
          text: 'Getting Started',
          items: [
            { text: 'Introduction', link: '/guide/' },
            { text: 'Installation', link: '/guide/installation' },
            { text: 'Quick Start', link: '/guide/quick-start' },
            { text: 'Configuration', link: '/guide/configuration' },
            { text: 'Authentication', link: '/guide/authentication' },
            { text: 'AI Agent Skill', link: '/guide/agents' },
          ]
        }
      ],
      '/reference/': [
        {
          text: 'Reference',
          items: [
            { text: 'CLI Commands', link: '/reference/cli' },
            { text: 'TUI Dashboard', link: '/reference/tui' },
            { text: 'Library API', link: '/reference/library' },
          ]
        }
      ],
      '/architecture/': [
        {
          text: 'Architecture',
          items: [
            { text: 'Overview', link: '/architecture/' },
            { text: 'Crate Structure', link: '/architecture/crates' },
            { text: 'Data Flow', link: '/architecture/data-flow' },
            { text: 'API Surface', link: '/architecture/api-surface' },
          ]
        }
      ],
      '/troubleshooting': [
        {
          text: 'Help',
          items: [
            { text: 'Troubleshooting', link: '/troubleshooting' },
          ]
        }
      ],
    },

    editLink: {
      pattern: 'https://github.com/hyperb1iss/unifly/edit/main/docs/:path',
      text: 'Edit this page on GitHub'
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/hyperb1iss/unifly' }
    ],

    footer: {
      message: 'Released under the Apache 2.0 License.',
      copyright: 'Copyright \u00a9 2025 Stefanie Jane'
    },

    search: {
      provider: 'local'
    }
  },

  markdown: {
    theme: {
      light: 'github-light',
      dark: 'one-dark-pro'
    }
  }
})
