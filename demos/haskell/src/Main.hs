{-# LANGUAGE DataKinds #-}
{-# LANGUAGE DerivingStrategies #-}
{-# LANGUAGE LambdaCase #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE PartialTypeSignatures #-}
{-# LANGUAGE QuasiQuotes #-}

module Main (main) where

import Auth.Biscuit.Servant
import Control.Monad.Error.Class (MonadError)
import Data.Time (getCurrentTime)
import Network.HTTP.Media.MediaType
import Network.Wai (Request, queryString)
import Network.Wai.Handler.Warp
import Picture
import Relude
import Servant
import Servant.API
import Servant.Server.Generic
import System.Directory (listDirectory)
import System.Environment (getEnv)
import System.Random (randomRIO)

generateBiscuit :: SecretKey -> IO (Biscuit Open Verified)
generateBiscuit secret =
  mkBiscuit
    secret
    [block|
    right("puna", "read");
    right("nix", "read");
  |]

main :: IO ()
main = do
  secret <- newSecret
  -- Just publicKey <- parsePublicKeyHex . fromString <$> getEnv "BISCUIT_PUBLIC_KEY"
  let publicKey = toPublic secret
  putStrLn "=== This token grants access to /protected/puna and /protected/nix ==="
  putBSLn . serializeB64 =<< generateBiscuit secret
  putStrLn "=== Starting server ==="
  runEnv 8080 $
    serveWithContext @API
      Proxy
      (genBiscuitCtxWith $ biscuitCfg publicKey)
      server

type API =
  (RequireBiscuit :> Routes)
    :<|> ("static" :> Raw)

type Routes = "protected" :> "dog" :> QueryParam' '[Required] "dog" Dog :> Get '[HTML, Picture] Pic

ambientChecks :: MonadIO m => m Authorizer
ambientChecks = do
  now <- liftIO getCurrentTime
  pure [authorizer|time({now});|]

server :: Server API
server =
  let handleAuth b =
        handleBiscuit b
          . withPriorityAuthorizerM ambientChecks
          . withPriorityAuthorizer [authorizer|allow if admin(true);|]
      handlers = getDogHandler
   in ( \b ->
          hoistServer @Routes Proxy (handleAuth b) getDogHandler
      )
        :<|> serveDirectoryWebApp "../node/assets/public"

getDogHandler :: Dog -> WithAuthorizer Handler Pic
getDogHandler dog =
  withAuthorizer
    [authorizer|
    dog({dog});
    allow if right({dog}, "read");
  |]
    $ do
      picture <- getPicture dog
      case picture of
        Nothing -> throwError err404
        Just bytes -> pure bytes
