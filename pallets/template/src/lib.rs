#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*,inherent::Vec};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn certificates)]
	pub(super) type Certificates<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A certificate added. [account, creator]
		CertificateAdded(T::AccountId, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// This certificate already exists.
		CertificateExist,
		/// You are not the admin.
		NotAdmin,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100)]
		pub fn add_certificate(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!Certificates::<T>::contains_key(&account), <Error<T>>::CertificateExist);

			let account_bytes: Vec<u8> = who.encode();
			// BOB
			// https://www.shawntabrizi.com/substrate-js-utilities/
			let match_bytes: Vec<u8> = hex_literal::hex!["8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"].into();

			ensure!(account_bytes == match_bytes, <Error<T>>::NotAdmin);

			Certificates::<T>::insert(&account, true);

			Self::deposit_event(Event::CertificateAdded(account, who));

			Ok(())
		}
	}
}
