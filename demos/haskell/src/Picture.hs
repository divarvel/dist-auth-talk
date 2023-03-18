{-# LANGUAGE LambdaCase #-}
{-# LANGUAGE OverloadedStrings #-}

module Picture where

import Auth.Biscuit.Servant
import Data.ByteString.Base64 (encodeBase64')
import Network.HTTP.Media.MediaType ((//))
import Network.Wai (Request (..))
import Relude
import Servant (Accept (..), FromHttpApiData (..))
import Servant.API (FromHttpApiData, MimeRender (..))
import System.Directory (listDirectory)
import System.Random (randomRIO)

newtype Pic = Pic {picBytes :: ByteString}

data HTML

instance Accept HTML where
  contentType _ = "text" // "html"

instance MimeRender HTML Pic where
  mimeRender _ (Pic picBytes) =
    fromStrict $
      "<img src=\"data:image/jpeg;base64," <> encodeBase64' picBytes <> "\"/>"

data Picture

instance Accept Picture where
  contentType _ = "image" // "jpg"

instance MimeRender Picture Pic where
  mimeRender _ = fromStrict . picBytes

readFromQueryString :: Request -> Either String ByteString
readFromQueryString req = maybeToRight "missing token query param" $ do
  (_, v) <- find ((== "token") . fst) $ queryString req
  v

biscuitCfg :: PublicKey -> BiscuitConfig String
biscuitCfg publicKey =
  let defaultCfg = defaultBiscuitConfig publicKey
   in defaultCfg
        { extractSerializedBiscuit = \req ->
            extractSerializedBiscuit defaultCfg req
              <|> readFromQueryString req
        }

data Dog = Puna | Nix

instance ToText Dog where
  toText = \case
    Puna -> "puna"
    Nix -> "nix"

instance FromHttpApiData Dog where
  parseUrlPiece = \case
    "puna" -> pure Puna
    "nix" -> pure Nix
    _ -> Left "Unknown dog (still a good dog, tho)"

instance ToTerm Dog a b where
  toTerm = toTerm . toText

getPicture :: MonadIO m => Dog -> m (Maybe Pic)
getPicture dog = do
  let dir =
        "../node/assets/" <> toString (toText dog) <> "/"
  pictures <- liftIO $ listDirectory dir
  filename <- (pictures !!?) <$> randomRIO (0, length pictures - 1)
  traverse (fmap Pic . readFileBS . (dir <>)) filename
