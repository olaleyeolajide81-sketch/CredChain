import { createSlice, createAsyncThunk, PayloadAction } from '@reduxjs/toolkit';
import { Address } from '@stellar/stellar-sdk';

interface User {
  address: Address;
  isConnected: boolean;
  isVerified: boolean;
}

interface AuthState {
  user: User | null;
  isLoading: boolean;
  error: string | null;
}

const initialState: AuthState = {
  user: null,
  isLoading: false,
  error: null,
};

// Async thunks
export const connectWallet = createAsyncThunk(
  'auth/connectWallet',
  async () => {
    try {
      // Connect to Freighter wallet
      const freighter = (window as any).freighter;
      if (!freighter) {
        throw new Error('Freighter wallet not found');
      }
      
      const publicKey = await freighter.getPublicKey();
      return { address: new Address(publicKey) };
    } catch (error) {
      throw new Error('Failed to connect wallet');
    }
  }
);

export const disconnectWallet = createAsyncThunk(
  'auth/disconnectWallet',
  async () => {
    // Disconnect logic
    return null;
  }
);

const authSlice = createSlice({
  name: 'auth',
  initialState,
  reducers: {
    clearError: (state: AuthState) => {
      state.error = null;
    },
    updateUserVerification: (state: AuthState, action: PayloadAction<boolean>) => {
      if (state.user) {
        state.user.isVerified = action.payload;
      }
    },
  },
  extraReducers: (builder: any) => {
    builder
      .addCase(connectWallet.pending, (state: AuthState) => {
        state.isLoading = true;
        state.error = null;
      })
      .addCase(connectWallet.fulfilled, (state: AuthState, action: any) => {
        state.isLoading = false;
        state.user = {
          address: action.payload.address,
          isConnected: true,
          isVerified: false,
        };
      })
      .addCase(connectWallet.rejected, (state: AuthState, action: any) => {
        state.isLoading = false;
        state.error = action.error.message || 'Failed to connect wallet';
      })
      .addCase(disconnectWallet.fulfilled, (state: AuthState) => {
        state.user = null;
        state.error = null;
      });
  },
});

export const { clearError, updateUserVerification } = authSlice.actions;
export default authSlice.reducer;
