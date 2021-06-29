import '@fontsource/roboto';
import './wasm_binding';

import * as React from 'react';
import ReactDOM from 'react-dom';

import { AppBar, Container, Link, makeStyles, Paper, Tab, Typography } from '@material-ui/core';
import { TabContext, TabList, TabPanel } from '@material-ui/lab';
import { ThemeProvider } from '@material-ui/styles';

import theme from './theme';

const classes = makeStyles(theme => ({
  root: {
    padding: theme.spacing(3, 2),
    height: 200,
    display: "flex",
    flexDirection: "column",
    justifyContent: "center"
  },
}));

function App() {
  const [value, setValue] = React.useState('1');
  const handleChange = (event, newValue) => {
    setValue(newValue);
  };

  return (
    <Container width="100%" maxWidth="100%">
      <ThemeProvider theme={theme}>
        <TabContext value={value}>
          <AppBar position="static">
            <TabList onChange={handleChange} aria-label="simple tabs example">
              <div className={classes.root}>
                <Typography variant="h5" color="textPrimary" display="inline">
                  <b>Aurora's Reactor Generator</b>
                </Typography>
              </div>
              <Tab label="Item One" value="1" />
              <Tab label="Item Two" value="2" />
              <Tab label="Item Three" value="3" />
            </TabList>
          </AppBar>
          <TabPanel value="1">Item One</TabPanel>
          <TabPanel value="2">Item Two</TabPanel>
          <TabPanel value="3">Item Three</TabPanel>
        </TabContext>
      </ThemeProvider>
    </Container>
  );
}

ReactDOM.render(<App />, document.querySelector('#app'));
