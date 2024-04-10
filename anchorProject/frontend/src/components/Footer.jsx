import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Container from '@mui/material/Container';
import Link from '@mui/material/Link';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import { makeStyles } from '@mui/styles';
import React from "react";
import TwitterIcon from "@mui/icons-material/Twitter";
import GitHubIcon from "@mui/icons-material/GitHub";

const useStyles = makeStyles((theme) => ({
  root: {
    backgroundColor: "transparent",
    boxShadow: "none",
    borderTop: "1px solid #e6e6e5",
    flexShrink: 0,
    marginTop: theme.spacing(10),
  },
  toolbar: {
    justifyContent: "space-between",
  },
  twitter: {
    marginRight: theme.spacing(1),
  },
}));

export default function Footer({ programID, voteAccount }) {
  const classes = useStyles();
  return (
    <AppBar position="static" className={classes.root}>
      <Container maxWidth="xl">
        <Toolbar className={classes.toolbar}>
          <Typography variant="caption">
            Made by{" "}
            <Link underline="always" href="https://twitter.com/adityarola">
              Aditya Rola
            </Link>
            {" | "}
            Powered by{" "}
            <Link underline="always" href="https://solana.com/">
              Solana
            </Link>
            {" | "}
            <Link
              underline="always"
              href={`https://explorer.solana.com/address/${programID.toString()}`}
            >
              Program ID
            </Link>
            {" | "}
            <Link
              underline="always"
              href={`https://explorer.solana.com/address/${voteAccount?.publicKey.toString()}`}
            >
              Vote Account
            </Link>
            {" | "}
            <Link underline="always" href="https://www.freepik.com">
              Icon Credits
            </Link>
          </Typography>
          <Box>
            <Link
              className={classes.twitter}
              href="https://twitter.com/adityarola"
            >
              <TwitterIcon />
            </Link>
            <Link href="https://github.com/adirola">
              <GitHubIcon />
            </Link>
          </Box>
        </Toolbar>
      </Container>
    </AppBar>
  );
}
