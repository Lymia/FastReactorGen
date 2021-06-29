import { createMuiTheme } from '@material-ui/core/styles';

const theme = createMuiTheme({
  palette: {
    primary: {
      main: '#ffccfa',
    },
    secondary: {
      main: '#cb8feb',
    },

    contrastThreshold: 3,
    tonalOffset: 0.2,
  },
});

export default theme;