module.exports = {
   content: [
      './templates/**/*.html',
      './static/js/**/*.js',
      '../../templates/**/*.html',
   ],
  darkMode: 'class', // or 'media' or 'class'
  theme: {
    extend: {
      colors: {
        // Paleta moderna
        'web': '#f8fafc',           // slate-50 - fondo claro
        'navbar': '#1e293b',        // slate-800 - navbar profesional
        'blog': '#e2e8f0',          // slate-200 - bordes sutiles
        'pagina': '#ffffff',        // blanco puro para contenido
        'accent': '#10b981',        // emerald-500 - color de acento (verde Obsidian)
        'accent-dark': '#34d399',   // emerald-400 - acento en dark mode
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'sans-serif'],
      },
      fontSize: {
        'xs': ['0.75rem', { lineHeight: '1.5' }],
        'sm': ['0.875rem', { lineHeight: '1.5715' }],
        'base': ['1rem', { lineHeight: '1.75' }],
        'lg': ['1.125rem', { lineHeight: '1.75' }],
        'xl': ['1.25rem', { lineHeight: '1.75' }],
        '2xl': ['1.5rem', { lineHeight: '1.415' }],
        '3xl': ['1.875rem', { lineHeight: '1.333' }],
        '4xl': ['2.25rem', { lineHeight: '1.277' }],
      },
      boxShadow: {
        'soft': '0 2px 15px -3px rgba(0, 0, 0, 0.07), 0 10px 20px -2px rgba(0, 0, 0, 0.04)',
        'card': '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
        'card-hover': '0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -5px rgba(0, 0, 0, 0.04)',
      },
      backdropBlur: {
        'xs': '2px',
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
