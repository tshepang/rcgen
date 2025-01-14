use std::fmt;

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
/// The error type of the rcgen crate
pub enum Error {
	/// The given certificate couldn't be parsed
	CouldNotParseCertificate,
	/// The given certificate signing request couldn't be parsed
	CouldNotParseCertificationRequest,
	/// The given key pair couldn't be parsed
	CouldNotParseKeyPair,
	#[cfg(feature = "x509-parser")]
	/// Invalid subject alternative name type
	InvalidNameType,
	/// An IP address was provided as a byte array, but the byte array was an invalid length.
	InvalidIpAddressOctetLength(usize),
	/// There is no support for generating
	/// keys for the given algorithm
	KeyGenerationUnavailable,
	#[cfg(feature = "x509-parser")]
	/// Unsupported extension requested in CSR
	UnsupportedExtension,
	/// The requested signature algorithm is not supported
	UnsupportedSignatureAlgorithm,
	/// Unspecified `ring` error
	RingUnspecified,
	/// The `ring` library rejected the key upon loading
	RingKeyRejected(String),
	/// The provided certificate's signature algorithm
	/// is incompatible with the given key pair
	CertificateKeyPairMismatch,
	/// Time conversion related errors
	Time,
	#[cfg(feature = "pem")]
	/// Error from the pem crate
	PemError(pem::PemError),
	/// Error generated by a remote key operation
	RemoteKeyError,
	/// Unsupported field when generating a CSR
	UnsupportedInCsr,
	/// Invalid certificate revocation list (CRL) next update.
	InvalidCrlNextUpdate,
	/// CRL issuer specifies Key Usages that don't include cRLSign.
	IssuerNotCrlSigner,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use self::Error::*;
		match self {
			CouldNotParseCertificate => write!(f, "Could not parse certificate")?,
			CouldNotParseCertificationRequest => write!(
				f,
				"Could not parse certificate signing \
				request"
			)?,
			CouldNotParseKeyPair => write!(f, "Could not parse key pair")?,
			#[cfg(feature = "x509-parser")]
			InvalidNameType => write!(f, "Invalid subject alternative name type")?,
			InvalidIpAddressOctetLength(actual) => {
				write!(f, "Invalid IP address octet length of {actual} bytes")?
			},
			KeyGenerationUnavailable => write!(
				f,
				"There is no support for generating \
				keys for the given algorithm"
			)?,
			UnsupportedSignatureAlgorithm => write!(
				f,
				"The requested signature algorithm \
				is not supported"
			)?,
			#[cfg(feature = "x509-parser")]
			UnsupportedExtension => write!(f, "Unsupported extension requested in CSR")?,
			RingUnspecified => write!(f, "Unspecified ring error")?,
			RingKeyRejected(e) => write!(f, "Key rejected by ring: {}", e)?,
			CertificateKeyPairMismatch => write!(
				f,
				"The provided certificate's signature \
				algorithm is incompatible with the given key pair"
			)?,

			Time => write!(f, "Time error")?,
			RemoteKeyError => write!(f, "Remote key error")?,
			#[cfg(feature = "pem")]
			PemError(e) => write!(f, "PEM error: {}", e)?,
			UnsupportedInCsr => write!(f, "Certificate parameter unsupported in CSR")?,
			InvalidCrlNextUpdate => write!(f, "Invalid CRL next update parameter")?,
			IssuerNotCrlSigner => write!(
				f,
				"CRL issuer must specify no key usage, or key usage including cRLSign"
			)?,
		};
		Ok(())
	}
}

impl std::error::Error for Error {}

impl From<ring::error::Unspecified> for Error {
	fn from(_unspecified: ring::error::Unspecified) -> Self {
		Error::RingUnspecified
	}
}

impl From<ring::error::KeyRejected> for Error {
	fn from(err: ring::error::KeyRejected) -> Self {
		Error::RingKeyRejected(err.to_string())
	}
}

#[cfg(feature = "pem")]
impl From<pem::PemError> for Error {
	fn from(e: pem::PemError) -> Self {
		Error::PemError(e)
	}
}
